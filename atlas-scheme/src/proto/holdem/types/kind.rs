use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TableStateKind {
    Waiting,    // 空闲状态：
    Preparing,  // 准备阶段：
    Battling,   // 对战阶段：
    Concluding, // 结算阶段：
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TableStreetKind {
    PreFlop, // 翻牌前（Pre-Flop）
    Flop,    // 翻牌圈（Flop）
    Turn,    // 转牌圈（Turn）
    River,   // 河牌圈（River）
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum GameActKind {
    Fold,
    Call,
    Check,
    Bet(u64),
    Raise(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AtlasSuitKind {
    Spade = 0,   // 黑桃
    Club = 1,    // 梅花
    Heart = 2,   // 红心
    Diamond = 3, // 方块
}


/// 点数（AtlasValue）枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AtlasValueKind {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}
