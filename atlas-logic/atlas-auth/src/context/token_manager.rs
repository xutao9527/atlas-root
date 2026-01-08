use dashmap::DashMap;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;
use tracing::debug;

/// token 默认有效期：1 小时
const TOKEN_TTL: Duration = Duration::from_secs(3600);
/// 后台清理间隔：60 秒
const CLEAN_INTERVAL: Duration = Duration::from_secs(60);
/// 清理标志
static CLEANER_STARTED: AtomicBool = AtomicBool::new(false);

/// 全局 { token ->   (uid, expire) }
static TOKEN_MAP: OnceLock<Arc<DashMap<String, (String, SystemTime)>>> = OnceLock::new();
/// 全局 { uid   ->   token }                   用于覆盖同账号旧 token
static UID_MAP: OnceLock<Arc<DashMap<String, String>>> = OnceLock::new();
/// 全局 { expire_time -> token)                优先队列,堆顶总是最早过期的 token
static EXPIRE_HEAP: OnceLock<Arc<Mutex<BinaryHeap<Reverse<(SystemTime, String)>>>>> = OnceLock::new();

/// 获取全局 TOKEN_MAP
fn token_map() -> &'static Arc<DashMap<String, (String, SystemTime)>> {
    TOKEN_MAP.get_or_init(|| Arc::new(DashMap::new()))
}

/// 获取全局 UID_MAP
fn uid_map() -> &'static Arc<DashMap<String, String>> {
    UID_MAP.get_or_init(|| Arc::new(DashMap::new()))
}

/// 获取全局 EXPIRE_HEAP
fn expire_heap() -> &'static Arc<Mutex<BinaryHeap<Reverse<(SystemTime, String)>>>> {
    EXPIRE_HEAP.get_or_init(|| Arc::new(Mutex::new(BinaryHeap::new())))
}

/// 存储 token，如果同一账号已有 token，先删除旧 token
/// 同时将 token 加入最小堆用于过期清理
/// 返回 expire_at（unix seconds）
pub async fn store_token(token: &str, uid: &str) -> Result<u64, &'static str> {
    let expire_at = SystemTime::now() + TOKEN_TTL;

    // 删除同一账号旧 token
    if let Some(old_token) = uid_map().get(uid).map(|r| r.value().clone()) {
        token_map().remove(&old_token);
    }

    // 存储新 token
    uid_map().insert(uid.to_string(), token.to_string());
    token_map().insert(token.to_string(), (uid.to_string(), expire_at));

    // 将 token 插入优先队列
    let mut heap = expire_heap().lock().await;
    heap.push(Reverse((expire_at, token.to_string())));

    let expire_at_unix = expire_at
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|_| "invalid system time")?
        .as_secs();

    Ok(expire_at_unix)
}

/// 验证 token 是否有效（Sliding TTL）
/// 成功：返回 (uid, expire_at_unix)
/// 失败：返回错误原因
pub async fn validate_token(token: &str) -> Result<(String, u64), &'static str> {
    match token_map().get_mut(token) {
        Some(mut entry) => {
            let now = SystemTime::now();
            let (uid, expire_at) = entry.value_mut();
            if *expire_at > now {
                // 1️⃣ Sliding TTL：续签
                let new_expire = now + TOKEN_TTL;
                *expire_at = new_expire;
                // 2️⃣ 推入 heap（旧 expire 由 cleaner 忽略）
                let mut heap = expire_heap().lock().await;
                heap.push(Reverse((new_expire, token.to_string())));
                // 3️⃣ 转成 unix seconds 返回
                let expire_at_unix = new_expire
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .map_err(|_| "invalid system time")?
                    .as_secs();

                Ok((uid.clone(), expire_at_unix))
            } else {
                // token 已过期，删除
                if let Some((uid, _)) = token_map().remove(token) {
                    uid_map().remove(&uid);
                }
                Err("token expired")
            }
        }
        None => Err("token not found"),
    }
}


/// 启动 tokio 异步后台任务，定期清理过期 token
/// 清理策略：只处理堆顶过期 token，性能高效
pub fn start_token_cleaner() {
    if CLEANER_STARTED.fetch_or(true, Ordering::SeqCst) {
        return;
    }

    tokio::spawn(async move{
        let token_map = token_map();
        let uid_map = uid_map();
        let heap = expire_heap();
        loop {
            tokio::time::sleep(Duration::from_secs(3)).await;
            debug!("================ TOKEN DEBUG ================");
            // 打印 token_map
            debug!("TOKEN_MAP:");
            for entry in token_map.iter() {
                let token = entry.key();
                let (uid, expire) = entry.value();
                let ttl = expire
                    .duration_since(SystemTime::now())
                    .map(|d| d.as_secs())
                    .unwrap_or(0);

                debug!(
                    "  token={} uid={} ttl={}s",
                    token, uid, ttl
                );
            }
            // 打印 uid_map
            debug!("UID_MAP:");
            for entry in uid_map.iter() {
                debug!(
                    "  uid={} -> token={}",
                    entry.key(),
                    entry.value()
                );
            }

            // 打印 heap（注意：这里只能 clone 一下用于调试）
            let heap_snapshot = {
                let heap_lock = heap.lock().await;
                heap_lock.clone()
            };

            debug!("EXPIRE_HEAP (top 5):");
            for Reverse((expire, token)) in heap_snapshot.into_iter().take(10) {
                let ttl = expire
                    .duration_since(SystemTime::now())
                    .map(|d| d.as_secs())
                    .unwrap_or(0);

                debug!(
                    "  token={} expire_in={}s",
                    token, ttl
                );
            }
        }
    });

    tokio::spawn(async move {
        let token_map = token_map().clone();
        let uid_map = uid_map().clone();
        let heap = expire_heap().clone();
        loop {
            tokio::time::sleep(CLEAN_INTERVAL).await;
            let now = SystemTime::now();
            let mut heap_lock = heap.lock().await;
            while let Some(Reverse((_, token))) = heap_lock.peek() {
                // 只处理 heap 顶部过期 token
                match token_map.get(token) {
                    Some(entry) => {
                        let actual_expire = entry.value().1;
                        if actual_expire <= now {
                            debug!("token {} expired", token);
                            // token 真的过期，直接 remove 返回 value
                            if let Some((uid, _)) = token_map.remove(token) {
                                uid_map.remove(&uid);
                            }
                            heap_lock.pop();
                        } else {
                            debug!("token {} is not expired yet", token);
                            break;
                        }
                    }
                    None => {
                        // token 已经被删除，直接 pop
                        heap_lock.pop();
                    }
                }
            }
        }
    });
}