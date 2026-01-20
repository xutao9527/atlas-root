use serde::{Deserialize, Serialize};
use crate::proto::holdem::types::{AtlasSuitKind, AtlasValueKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableView {
    pub id: String,
    pub seats: Vec<Option<String>>, // Some(nickname) / None
    pub small_blind_amount: u64,
    pub big_blind_amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerView{
    pub id: String,       // 玩家全局唯一 ID
    pub nickname: String, // 玩家昵称
    pub balance: u64,     // 玩家当前可用筹码

    pub sit_out: bool,            // 是否下局不玩（离桌）
    pub win: bool,                // 是不是赢家
    pub is_active: bool,          // 是否仍在牌局中（Fold 后为 false）
    pub has_acted: bool,          // 本下注轮是否已经行动过
    pub acted_view: String, // 行动结果展示
    pub is_all_in: bool,          // 是否已all_in
    pub street_bet: u64,          // 本下注轮（当前 street）中已投入的筹码
    pub total_bet: u64,           // 总投入筹码
}

/// 扑克牌结构体
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AtlasCardView {
    /// 牌的花色
    pub suit: AtlasSuitKind,
    /// 牌的点数
    pub value: AtlasValueKind,
}