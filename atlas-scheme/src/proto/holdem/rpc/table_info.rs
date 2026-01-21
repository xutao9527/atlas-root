use serde::{Deserialize, Serialize};
use crate::proto::holdem::types::{AtlasCardView, PlayerAvailableActView, PlayerView, TableStateKind, TableStreetKind};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetTableInfoReq{
    pub table_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetTableInfoResp {
    pub id: String,                                     // 桌子的唯一 ID（生命周期贯穿整个桌子）
    pub seats: Vec<Option<PlayerView>>,                 // 桌子上的固定座位数组
    pub state: TableStateKind,                          // 当前桌子的状态机状态
    pub hand_id: String,                                // 当前这一局（hand）的唯一标识
    pub street: TableStreetKind,                        // 当前所处的下注阶段（Street）
    pub small_blind_amount: u64,                        // 小盲注金额（桌子级别的固定规则）
    pub big_blind_amount: u64,                          // 大盲注金额（桌子级别的固定规则）
    pub pot: u64,                                       // 当前局已经进入底池的总金额
    pub current_bet: u64,                               // 当前下注轮中需要跟注的最大下注额

    pub dealer_pos: usize,                              // 当前局庄家按钮所在的座位索引
    pub small_blind_pos: usize,                         // 当前局小盲注所在的座位索引
    pub big_blind_pos: usize,                           // 当前局大盲注所在的座位索引
    pub current_turn: usize,                            // 当前轮到行动的座位索引
    pub current_turn_act: PlayerAvailableActView,       // 当前轮到行动座位可用动作
    pub last_raiser_pos: usize,                         // 当前下注轮中，最后一次加注的玩家位置
    pub community_cards: [Option<AtlasCardView>; 5],    // 公共牌（Community Cards），最多 5 张

    pub hand_cards: [Option<AtlasCardView>; 2],         // 玩家手牌（底牌）
    pub seat_index:usize,
}
