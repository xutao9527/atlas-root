use atlas_core::net::rpc::packet_payload::AtlasWireError;
use serde_value::Value;

#[derive(Debug)]
pub enum TableError {
    InvalidSeat,        // 座位索引非法（超出 0..10）
    SeatOccupied,       // 该座位已经有玩家
    AlreadySeated,      // 玩家已在桌上
    InvalidState,       // 当前桌子状态不允许该操作
    NotEnoughPlayers,   // 坐下的玩家数量不足，无法开始一局
    InvalidAction,      // 动作在当前下注状态下不合法（如非法 check）
    InvalidBuyIn {
        min: u64,
        max: u64,
        actual: u64,
    },
}

impl From<TableError> for AtlasWireError {
    fn from(e: TableError) -> Self {
        match e {
            TableError::InvalidSeat => AtlasWireError {
                code: 40001,
                message: "invalid seat index".into(),
                data: None,
            },

            TableError::SeatOccupied => AtlasWireError {
                code: 40901,
                message: "seat already occupied".into(),
                data: None,
            },
            TableError::AlreadySeated => Self {
                code: 409,
                message: "player already seated".into(),
                data: None,
            },
            TableError::InvalidState => AtlasWireError {
                code: 40902,
                message: "operation not allowed in current table state".into(),
                data: None,
            },

            TableError::NotEnoughPlayers => AtlasWireError {
                code: 40002,
                message: "not enough players to start game".into(),
                data: None,
            },

            TableError::InvalidAction => AtlasWireError {
                code: 40003,
                message: "invalid action for current betting state".into(),
                data: None,
            },

            TableError::InvalidBuyIn { min, max, actual } => AtlasWireError {
                code: 40004,
                message: format!("buy-in {} not in allowed range ({} - {})", actual, min, max),
                data: Some(Value::Map(
                    [
                        (Value::String("min".into()), Value::U64(min)),
                        (Value::String("max".into()), Value::U64(max)),
                        (Value::String("actual".into()), Value::U64(actual)),
                    ]
                    .into_iter()
                    .collect(),
                )),
            },
        }
    }
}
