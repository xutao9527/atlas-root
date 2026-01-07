use crate::context::get_db;
use atlas_core::net::rpc::packet_header::AtlasWireKind::{ResponseErr, ResponseOk};
use atlas_core::net::rpc::packet_message::AtlasWireMessage;
use atlas_scheme::dto::auth_model::{LoginReq, LoginResp, RegisterReq, RegisterResp};
use atlas_scheme::model::atlas_user;
use atlas_scheme::model::sea_orm_active_enums::UserType;
use sea_orm::{ActiveModelTrait, IntoActiveModel};
use tracing::log;
use ulid::Ulid;

pub async fn register(request: AtlasWireMessage<RegisterReq>) -> AtlasWireMessage<RegisterResp> {
    let register_req = request.payload;
    let model = atlas_user::Model {
        id: Ulid::new().to_string(),
        user_type: Option::from(UserType::Normal),
        account: register_req.account,
        password: register_req.password,
        name: register_req.nickname,
        balance: Default::default(),
        avatar: None,
        created_at: chrono::Utc::now(),
        updated_at: None,
    };
    let active_model = model.into_active_model();
    let res = active_model.insert(get_db()).await;
    match res {
        Ok(_) => {
            AtlasWireMessage {
                header: request.header.with_kind(ResponseOk),
                payload: RegisterResp {
                    ok: true,
                    message: Some("register success".into()),
                }
            }
        }
        Err(e) => {
            log::error!("register error: {:?}", e);
            AtlasWireMessage {
                header: request.header.with_kind(ResponseErr),
                payload: RegisterResp {
                    ok: false,
                    message: Some(format!("register failed: {}", e)),
                }
            }
        }
    }
}

pub async fn login(request: AtlasWireMessage<LoginReq>) -> AtlasWireMessage<LoginResp> {
    let token = format!("{}|{}", request.payload.account, request.payload.password);
    AtlasWireMessage {
        header: request.header.with_kind(ResponseOk),
        payload: LoginResp {
            ok: true,
            token: Option::from(token),
            error: None,
        },
    }
}
