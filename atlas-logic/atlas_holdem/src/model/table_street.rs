#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TableStreet {
    PreFlop, // 翻牌前（Pre-Flop）
    Flop,    // 翻牌圈（Flop）
    Turn,    // 转牌圈（Turn）
    River,   // 河牌圈（River）
}
