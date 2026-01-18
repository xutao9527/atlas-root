use rand::prelude::*;
use crate::model::card::{AtlasCard, AtlasSuit, AtlasValue};
use crate::model::deck::AtlasDeck;

impl AtlasDeck {
    /// 创建一副标准 52 张扑克牌（不含大小王）
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for &suit in &[AtlasSuit::Heart, AtlasSuit::Diamond, AtlasSuit::Club, AtlasSuit::Spade] {
            for &value in &[
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
                cards.push(AtlasCard { suit, value });
            }
        }
        AtlasDeck { cards }
    }

    /// 洗牌（随机打乱牌堆）
    pub fn shuffle(&mut self) {
        // 恢复成完整 52 张
        self.cards.clear();
        for &suit in &[AtlasSuit::Heart, AtlasSuit::Diamond, AtlasSuit::Club, AtlasSuit::Spade] {
            for &value in &[
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
                self.cards.push(AtlasCard { suit, value });
            }
        }
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