use sea_orm::prelude::Decimal;


use serde::{Deserialize, Serialize};
use atlas_core::net::core::rpc::AtlasModuleId;
use atlas_core::net::protocol::frame_body_notify::AtlasNotifySpec;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserUpdateNotify {
    pub id: String,
    pub account: String,
    pub name: String,
    pub balance: Decimal,
    pub avatar: Option<String>,
}

impl AtlasNotifySpec for UserUpdateNotify {
    const MODULE_ID: AtlasModuleId = AtlasModuleId::Auth;
    const NOTIFY_TYPE_ID: u16 = 1;
}