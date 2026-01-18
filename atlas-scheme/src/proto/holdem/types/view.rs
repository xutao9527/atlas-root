use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableView {
    pub id: String,
    pub seats: Vec<Option<String>>, // Some(nickname) / None
    pub small_blind_amount: u64,
    pub big_blind_amount: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GameAct {
    Fold,
    Call,
    Check,
    Bet(u64),
    Raise(u64),
}
