use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct TsFieldCtx {
    pub name: String,
    pub ts_type: String,
}