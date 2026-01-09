use std::fmt;
use rand::prelude::*;

/// 花色（Suit）枚举
/// - Hearts: 红心
/// - Diamonds: 方块
/// - Clubs: 梅花
/// - Spades: 黑桃
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Suit::Hearts   => "♥️",
            Suit::Diamonds => "♦️",
            Suit::Clubs    => "♣️",
            Suit::Spades   => "♠️",
        };
        write!(f, "{}", s)
    }
}

/// 点数（Rank）枚举
/// - Two ~ Ace 表示从 2 到 A 的牌
/// - 实现 PartialOrd/Ord，可用于比较大小
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Rank::Two   => "2",
            Rank::Three => "3",
            Rank::Four  => "4",
            Rank::Five  => "5",
            Rank::Six   => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine  => "9",
            Rank::Ten   => "10",
            Rank::Jack  => "J",
            Rank::Queen => "Q",
            Rank::King  => "K",
            Rank::Ace   => "A",
        };
        write!(f, "{}", s)
    }
}


/// 扑克牌结构体
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    /// 牌的花色
    pub suit: Suit,
    /// 牌的点数
    pub rank: Rank,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 输出例子： "K♣" 或 "10♦"
        write!(f, "{:>2}{:>2}", self.suit, self.rank)
    }
}

/// 一副扑克牌（52 张）
/// - 内部使用 Vec<Card> 保存牌
/// - 可实现洗牌、发牌等方法
pub struct Deck {
    /// 当前牌堆
    cards: Vec<Card>,
}

impl Deck {
    /// 创建一副标准 52 张扑克牌（不含大小王）
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for &rank in &[
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
                Rank::Ace,
            ] {
                cards.push(Card { suit, rank });
            }
        }
        Deck { cards }
    }

    /// 洗牌（随机打乱牌堆）
    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    /// 发一张牌（从牌堆顶部弹出）
    /// - 返回 Option<Card>
    pub fn deal_one(&mut self) -> Option<Card> {
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
