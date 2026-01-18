use crate::model::card::AtlasCard;

/// 一副扑克牌（52 张）
/// - 内部使用 Vec<Card> 保存牌
/// - 可实现洗牌、发牌等方法
pub struct AtlasDeck {
    /// 当前牌堆
    pub cards: Vec<AtlasCard>,
}
