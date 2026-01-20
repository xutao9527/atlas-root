use crate::model::card::AtlasCard;
use crate::model::deck::AtlasDeck;
use crate::model::player::Player;
use crate::model::table_state::TableState;
use crate::model::table_street::TableStreet;

pub struct Table {
    pub id: String,                              // 桌子的唯一 ID（生命周期贯穿整个桌子）
    pub seats: Vec<Option<Player>>,              // 桌子上的固定座位数组
    pub state: TableState,                       // 当前桌子的状态机状态
    pub hand_id: String,                         // 当前这一局（hand）的唯一标识
    pub street: TableStreet,                     // 当前所处的下注阶段（Street）
    pub small_blind_amount: u64,                 // 小盲注金额（桌子级别的固定规则）
    pub big_blind_amount: u64,                   // 大盲注金额（桌子级别的固定规则）
    pub pot: u64,                                // 当前局已经进入底池的总金额
    pub current_bet: u64,                        // 当前下注轮中需要跟注的最大下注额
    pub dealer_pos: usize,                       // 当前局庄家按钮所在的座位索引
    pub small_blind_pos: usize,                  // 当前局小盲注所在的座位索引
    pub big_blind_pos: usize,                    // 当前局大盲注所在的座位索引
    pub current_turn: usize,                     // 当前轮到行动的座位索引
    pub last_raiser_pos: usize,                  // 当前下注轮中，最后一次加注的玩家位置
    pub deck: AtlasDeck,                         // 当前局的牌堆
    pub community_cards: [Option<AtlasCard>; 5], // 公共牌（Community Cards），最多 5 张

    pub act_log: Vec<String>,           // 当前下注阶段日志
    pub street_log: Vec<TableStreet>,   // 当前下注阶段日志
}
