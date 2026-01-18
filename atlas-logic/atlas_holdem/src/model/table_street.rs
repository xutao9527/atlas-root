use atlas_scheme::proto::holdem::types::TableStreetKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TableStreet {
    PreFlop, // 翻牌前（Pre-Flop）
    Flop,    // 翻牌圈（Flop）
    Turn,    // 转牌圈（Turn）
    River,   // 河牌圈（River）
}

impl From<TableStreet> for TableStreetKind {
    fn from(street: TableStreet) -> Self {
        match street {
            TableStreet::PreFlop => TableStreetKind::PreFlop,
            TableStreet::Flop    => TableStreetKind::Flop,
            TableStreet::Turn    => TableStreetKind::Turn,
            TableStreet::River   => TableStreetKind::River,
        }
    }
}