use crate::context::get_db;
use atlas_core::net::rpc::packet_header::AtlasWireKind::{ResponseErr, ResponseOk};
use atlas_core::net::rpc::packet_message::AtlasWireMessage;
use atlas_scheme::dto::auth_model::{LoginReq, LoginResp, RegisterReq, RegisterResp};
use atlas_scheme::model::atlas_user;
use atlas_scheme::model::sea_orm_active_enums::UserType;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel};
use sea_orm::{QueryFilter};
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
        Ok(_) => AtlasWireMessage {
            header: request.header.with_kind(ResponseOk),
            payload: RegisterResp {
                ok: true,
                message: Some("register success".into()),
            },
        },
        Err(e) => {
            log::error!("register error: {:?}", e);
            AtlasWireMessage {
                header: request.header.with_kind(ResponseErr),
                payload: RegisterResp {
                    ok: false,
                    message: Some(format!("register failed: {}", e)),
                },
            }
        }
    }
}

pub async fn login(request: AtlasWireMessage<LoginReq>) -> AtlasWireMessage<LoginResp> {
    let login_req = request.payload;
    let result = atlas_user::Entity::find()
        .filter(atlas_user::Column::Account.eq(login_req.account))
        .one(get_db())
        .await;
    match result {
        Ok(Some(user)) => {
            if user.password != login_req.password {
                return AtlasWireMessage {
                    header: request.header.with_kind(ResponseErr),
                    payload: LoginResp {
                        ok: false,
                        token: None,
                        error: Some("Invalid password".into()),
                    },
                };
            }
            AtlasWireMessage {
                header: request.header.with_kind(ResponseOk),
                payload: LoginResp {
                    ok: true,
                    token: Some(Ulid::new().to_string()),
                    error: None,
                },
            }
        }
        Ok(None) => AtlasWireMessage {
            header: request.header.with_kind(ResponseErr),
            payload: LoginResp {
                ok: false,
                token: None,
                error: Some("account not found".into()),
            },
        },
        Err(e) => AtlasWireMessage {
            header: request.header.with_kind(ResponseErr),
            payload: LoginResp {
                ok: false,
                token: None,
                error: Some(format!("Login failed: {}", e)),
            },
        },
    }
}
