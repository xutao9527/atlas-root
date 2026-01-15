use rs_poker::core::Rank;
use crate::model::card::{AtlasCard};

#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,                             // 玩家全局唯一 ID
    pub nickname: String,                       // 玩家昵称
    pub balance: u64,                           // 玩家当前可用筹码

    pub hand_cards: [Option<AtlasCard>; 2],     // 玩家手牌（底牌），每人 2 张
    
    pub cards_str: String,

    pub sit_out: bool,                          // 是否下局不玩（离桌）
    pub win: bool,                              // 是不是赢家
    pub cards_rank: Option<Rank>,               // 牌力值
    pub is_active: bool,                        // 是否仍在牌局中（Fold 后为 false）
    pub has_acted: bool,                        // 本下注轮是否已经行动过
    pub is_all_in: bool,                        // 是否已all_in
    pub street_bet: u64,                        // 本下注轮（当前 street）中已投入的筹码
    pub total_bet: u64                          // 总投入筹码
}

impl Player {
    pub fn reset(&mut self) {
        self.hand_cards = [None; 2];                                                    // 更新手牌
        self.cards_str = "".to_string();                                                // 更新手牌显示
        self.win = false;                                                               // 更新输赢状态
        self.cards_rank = None;                                                         // 更新牌力
        self.is_active = true;                                                          // 更新玩家状态
        self.has_acted = false;                                                         // 更新行动状态
        self.is_all_in = false;                                                         // 更新all_in状态
        self.street_bet = 0;                                                            // 更新本轮投注额
        self.total_bet = 0;                                                             // 更新总投入筹码
    }
}

#[derive(Debug)]
pub enum PlayerAction {
    Fold,
    Call,
    Check,
    Bet(u64),
    Raise(u64),
}