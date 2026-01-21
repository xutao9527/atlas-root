use crate::model::table::Table;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;

pub struct TableManager {
    tables: DashMap<String, Arc<RwLock<Table>>>,
}

impl TableManager {

    /// 创建一个空的 TableManager
    pub fn new() -> Self {
        Self {
            tables: DashMap::new(),
        }
    }

    /// 初始化 N 张桌子（服务启动时调用）
    pub fn init_tables(&self, count: usize) {
        for _i in 0..count {
            let table = Table::new_six(10, 20);
            debug!("init table {}",table.id);
            self.tables.insert(table.id.clone(), Arc::new(RwLock::new(table)));
        }
    }

    /// 获取指定 table
    pub fn get(&self, table_id: &str) ->  Option<Arc<RwLock<Table>>> {
        self.tables.get(table_id).map(|e| e.value().clone())
    }

    /// 返回所有 table（只读场景）
    pub fn all(&self) -> Vec<Arc<RwLock<Table>>> {
        self.tables
            .iter()
            .map(|e| Arc::clone(e.value()))
            .collect()
    }

    /// 添加 table（动态扩容）
    pub fn insert(&self, table: Table) {
        let table_id = table.id.clone();
        self.tables
            .insert(table_id, Arc::new(RwLock::new(table)));
    }

    /// 当前 table 数量
    pub fn len(&self) -> usize {
        self.tables.len()
    }

    /// 是否存在 table
    pub fn contains(&self, table_id: &str) -> bool {
        self.tables.contains_key(table_id)
    }
}
