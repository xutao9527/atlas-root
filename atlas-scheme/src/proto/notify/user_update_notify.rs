use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserUpdateNotify {
    pub id: String,
    pub account: String,
    pub name: String,
    pub balance: Decimal,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserUpdateNotify1 {
    pub id: String,
    pub account: String,
    pub name: String,
    pub balance: Decimal,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserUpdateNotify2 {
    pub id: String,
    pub account: String,
    pub name: String,
    pub balance: Decimal,
    pub avatar: Option<String>,
}
