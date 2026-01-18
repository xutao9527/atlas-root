use atlas_scheme::proto::holdem::types::{AtlasCardView, AtlasSuitKind, AtlasValueKind};

/// 花色（AtlasSuit）枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtlasSuit {
    Spade = 0,   // 黑桃
    Club = 1,    // 梅花
    Heart = 2,   // 红心
    Diamond = 3, // 方块
}

impl From<AtlasSuit> for AtlasSuitKind {
    fn from(s: AtlasSuit) -> Self {
        match s {
            AtlasSuit::Spade   => AtlasSuitKind::Spade,
            AtlasSuit::Club    => AtlasSuitKind::Club,
            AtlasSuit::Heart   => AtlasSuitKind::Heart,
            AtlasSuit::Diamond => AtlasSuitKind::Diamond,
        }
    }
}

impl From<&AtlasSuit> for AtlasSuitKind {
    fn from(s: &AtlasSuit) -> Self {
        (*s).into()
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

impl From<AtlasValue> for AtlasValueKind {
    fn from(v: AtlasValue) -> Self {
        match v {
            AtlasValue::Two   => AtlasValueKind::Two,
            AtlasValue::Three => AtlasValueKind::Three,
            AtlasValue::Four  => AtlasValueKind::Four,
            AtlasValue::Five  => AtlasValueKind::Five,
            AtlasValue::Six   => AtlasValueKind::Six,
            AtlasValue::Seven => AtlasValueKind::Seven,
            AtlasValue::Eight => AtlasValueKind::Eight,
            AtlasValue::Nine  => AtlasValueKind::Nine,
            AtlasValue::Ten   => AtlasValueKind::Ten,
            AtlasValue::Jack  => AtlasValueKind::Jack,
            AtlasValue::Queen => AtlasValueKind::Queen,
            AtlasValue::King  => AtlasValueKind::King,
            AtlasValue::Ace   => AtlasValueKind::Ace,
        }
    }
}

impl From<&AtlasValue> for AtlasValueKind {
    fn from(v: &AtlasValue) -> Self {
        (*v).into()
    }
}

/// 扑克牌结构体
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtlasCard {
    /// 牌的花色
    pub suit: AtlasSuit,
    /// 牌的点数
    pub value: AtlasValue,
}


impl From<&AtlasCard> for AtlasCardView {
    fn from(card: &AtlasCard) -> Self {
        AtlasCardView {
            suit: card.suit.into(),
            value: card.value.into(),
        }
    }
}

impl From<AtlasCard> for AtlasCardView {
    fn from(card: AtlasCard) -> Self {
        (&card).into()
    }
}