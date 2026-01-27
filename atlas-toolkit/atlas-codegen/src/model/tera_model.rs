use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct TeraFieldCtx {
    pub name: String,
    pub ts_type: String,
}

#[derive(Serialize)]
pub struct TsEnumVariant {
    pub name: String,
    pub payload: Option<String>,
}

#[derive(Serialize)]
pub  struct TsEnumNumberVariant {
    pub name: String,
    pub value: String,
}