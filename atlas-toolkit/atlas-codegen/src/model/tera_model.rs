use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct TeraFieldCtx {
    pub name: String,
    pub ts_type: String,
}