use std::fmt;
use rand::prelude::*;

/// 花色（AtlasSuit）枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtlasSuit {
    /// 黑桃
    Spade = 0,
    /// 梅花
    Club = 1,
    /// 红心
    Heart = 2,
    /// 方块
    Diamond = 3,
}

impl fmt::Display for AtlasSuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AtlasSuit::Heart   => "♥️",
            AtlasSuit::Diamond => "♦️",
            AtlasSuit::Club    => "♣️",
            AtlasSuit::Spade   => "♠️",
        };
        write!(f, "{}", s)
    }
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

impl fmt::Display for AtlasValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AtlasValue::Two   => "2",
            AtlasValue::Three => "3",
            AtlasValue::Four  => "4",
            AtlasValue::Five  => "5",
            AtlasValue::Six   => "6",
            AtlasValue::Seven => "7",
            AtlasValue::Eight => "8",
            AtlasValue::Nine  => "9",
            AtlasValue::Ten   => "T",
            AtlasValue::Jack  => "J",
            AtlasValue::Queen => "Q",
            AtlasValue::King  => "K",
            AtlasValue::Ace   => "A",
        };
        write!(f, "{}", s)
    }
}


/// 扑克牌结构体
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtlasCard {
    /// 牌的花色
    pub suit: AtlasSuit,
    /// 牌的点数
    pub rank: AtlasValue,
}

impl fmt::Display for AtlasCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 输出例子： "K♣" 或 "10♦"
        write!(f, "{:>2}{:>2}", self.suit, self.rank)
    }
}

/// 一副扑克牌（52 张）
/// - 内部使用 Vec<Card> 保存牌
/// - 可实现洗牌、发牌等方法
pub struct AtlasDeck {
    /// 当前牌堆
    cards: Vec<AtlasCard>,
}

impl AtlasDeck {
    /// 创建一副标准 52 张扑克牌（不含大小王）
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for &suit in &[AtlasSuit::Heart, AtlasSuit::Diamond, AtlasSuit::Club, AtlasSuit::Spade] {
            for &rank in &[
                AtlasValue::Two,
                AtlasValue::Three,
                AtlasValue::Four,
                AtlasValue::Five,
                AtlasValue::Six,
                AtlasValue::Seven,
                AtlasValue::Eight,
                AtlasValue::Nine,
                AtlasValue::Ten,
                AtlasValue::Jack,
                AtlasValue::Queen,
                AtlasValue::King,
                AtlasValue::Ace,
            ] {
                cards.push(AtlasCard { suit, rank });
            }
        }
        AtlasDeck { cards }
    }

    /// 洗牌（随机打乱牌堆）
    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    /// 发一张牌（从牌堆顶部弹出）
    /// - 返回 Option<Card>
    pub fn deal_one(&mut self) -> Option<AtlasCard> {
        self.cards.pop()
    }

    /// 剩余牌数量
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// 牌堆是否为空
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}
