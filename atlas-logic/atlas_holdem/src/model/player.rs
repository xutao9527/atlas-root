use crate::model::card::{AtlasCard};

#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,                             // 玩家全局唯一 ID
    pub nickname: String,                       // 玩家昵称
    pub balance: u64,                           // 玩家当前可用筹码
    pub hole_cards: [Option<AtlasCard>; 2],     // 玩家手牌（底牌），每人 2 张

    pub is_active: bool,                        // 是否仍在牌局中（Fold 后为 false）
    pub has_acted: bool,                        // 本下注轮是否已经行动过
    pub is_all_in: bool,                        // 是否已all_in
    pub street_bet: u64,                        // 本下注轮（当前 street）中已投入的筹码
}

#[derive(Debug)]
pub enum PlayerAction {
    Fold,
    Call,
    Check,
    Bet(u64),
    Raise(u64),
}