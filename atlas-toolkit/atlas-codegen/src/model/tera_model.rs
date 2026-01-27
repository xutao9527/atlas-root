use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize,Debug)]
pub struct TsTypeInfo {
    pub ts_type: String,          // "(PlayerView | null)[]"
    pub is_composite: bool,       // true
    pub imports: Vec<String>,     // ["PlayerView"]
}

#[derive(Serialize,Deserialize,Debug)]
pub struct TsFieldCtx {
    pub name: String,
    pub ts_type: String,
}

#[derive(Serialize,Debug)]
pub struct TsEnumVariant {
    pub name: String,
    pub payload: Option<String>,
}

#[derive(Serialize,Debug)]
pub  struct TsEnumNumberVariant {
    pub name: String,
    pub value: String,
}

