use crate::proto::notify::*;
use atlas_core::atlas_notify_specs;

atlas_notify_specs! {
    module notify {
        UserUpdateNotify = (AtlasModuleId::Auth, 1),
        UserUpdateNotify1 = (AtlasModuleId::Auth, 2),
        UserUpdateNotify2 = (AtlasModuleId::Auth, 3),
    }
}
