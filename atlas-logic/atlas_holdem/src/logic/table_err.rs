use serde_value::Value;
use atlas_core::net::rpc::packet_payload::AtlasWireError;
use crate::model::table_err::TableError;

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
            TableError::PlayerNotAtTable => Self {
                code: 409,
                message: "player not at seated".into(),
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
