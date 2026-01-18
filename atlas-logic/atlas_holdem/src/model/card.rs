/// 花色（AtlasSuit）枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtlasSuit {
    Spade = 0,   // 黑桃
    Club = 1,    // 梅花
    Heart = 2,   // 红心
    Diamond = 3, // 方块
}

/// 点数（AtlasValue）枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AtlasValue {
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

/// 扑克牌结构体
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtlasCard {
    /// 牌的花色
    pub suit: AtlasSuit,
    /// 牌的点数
    pub value: AtlasValue,
}
