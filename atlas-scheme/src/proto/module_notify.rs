use atlas_core::atlas_notify_specs;
use atlas_core::net::core::AtlasModuleId;
use crate::proto::notify::*;

atlas_notify_specs! {
        UserUpdateNotify => (AtlasModuleId::Auth, 1),
        UserUpdateNotify2 => (AtlasModuleId::Auth, 1),
}

