use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BasicAuthReq {
    pub account: String,
    pub password: String,
}
