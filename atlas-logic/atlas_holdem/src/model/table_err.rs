#[derive(Debug)]
pub enum TableError {
    InvalidSeat,      // 座位索引非法（超出 0..10）
    SeatOccupied,     // 该座位已经有玩家
    AlreadySeated,    // 玩家已在桌上
    PlayerNotAtTable, // 玩家没在桌上
    InvalidState,     // 当前桌子状态不允许该操作
    NotEnoughPlayers, // 坐下的玩家数量不足，无法开始一局
    InvalidAction,    // 动作在当前下注状态下不合法（如非法 check）
    InvalidBuyIn { min: u64, max: u64, actual: u64 },
}
