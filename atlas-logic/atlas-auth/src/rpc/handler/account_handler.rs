use crate::context::get_db;
use atlas_core::net::rpc::packet_header::AtlasWireKind::{ResponseErr, ResponseOk};
use atlas_core::net::rpc::packet_message::AtlasWireMessage;
use atlas_scheme::dto::auth_model::{AuthResp, BasicAuthReq, RegisterReq, RegisterResp, TokenAuthReq};
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

pub async fn basic_auth(request: AtlasWireMessage<BasicAuthReq>) -> AtlasWireMessage<AuthResp> {
    let login_req = request.payload;
    let result = atlas_user::Entity::find()
        .filter(atlas_user::Column::Account.eq(login_req.account))
        .one(get_db())
        .await;
    let mut login_resp = AtlasWireMessage {
        header: request.header.with_kind(ResponseErr),
        payload: AuthResp {
            ok: false,
            uid: None,
            token: None,
            error: None,
        },
    };
    match result {
        Ok(Some(user)) => {
            if user.password != login_req.password {
                login_resp.payload.error = Some("Invalid password".into());
                return login_resp;
            }
            login_resp.payload.ok = true;
            login_resp.payload.uid = Some(user.id);
            login_resp.payload.token = Some(Ulid::new().to_string());
            login_resp.header.kind = ResponseOk;
        }
        Ok(None) =>  {
            login_resp.payload.error = Some("account not found".into());

        },
        Err(e) => {
            login_resp.payload.error = Some(format!("auth failed: {}", e));
        },
    };
    login_resp
}

pub async fn token_auth(request: AtlasWireMessage<TokenAuthReq>) -> AtlasWireMessage<AuthResp> {
    let mut login_resp = AtlasWireMessage {
        header: request.header.with_kind(ResponseErr),
        payload: AuthResp {
            ok: false,
            uid: None,
            token: None,
            error: None,
        },
    };
    login_resp
}
