use atlas_scheme::proto::holdem::types::TableStateKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TableState {
    Waiting,    // 空闲状态：
    Preparing,  // 准备阶段：
    Battling,   // 对战阶段：
    Concluding, // 结算阶段：
}


impl From<TableState> for TableStateKind {
    fn from(state: TableState) -> Self {
        match state {
            TableState::Waiting    => TableStateKind::Waiting,
            TableState::Preparing  => TableStateKind::Preparing,
            TableState::Battling   => TableStateKind::Battling,
            TableState::Concluding => TableStateKind::Concluding,
        }
    }
}