#[derive(Debug, Clone)]
pub struct Player {
    pub id: u32,
    pub nickname: String,
    pub balance: u64,
}

/// 桌子的不同状态
#[derive(Debug, PartialEq)]
pub enum TableState {
    Waiting,
    Preparing,
    Battling,
    Concluding,
}

pub struct Table {
    pub table_id: u32,
    pub seats: [Option<Player>; 10],
    pub state: TableState,
}
