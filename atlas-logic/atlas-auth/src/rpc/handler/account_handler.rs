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
use crate::context::token_manager::{store_token, validate_token};

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
    let basic_auth_req = request.payload;
    let result = atlas_user::Entity::find()
        .filter(atlas_user::Column::Account.eq(basic_auth_req.account))
        .one(get_db())
        .await;
    let mut resp = AtlasWireMessage {
        header: request.header.with_kind(ResponseErr),
        payload: AuthResp {
            ok: false,
            uid: None,
            token: None,
            expire_at: None,
            error: None,
        },
    };
    match result {
        Ok(Some(user)) => {
            if user.password != basic_auth_req.password {
                resp.payload.error = Some("Invalid password".into());
                return resp;
            }
            let token = Ulid::new().to_string();
            match store_token(token.as_str(), user.id.as_str()).await {
                Ok(expire_at) => {
                    resp.payload.ok = true;
                    resp.payload.uid = Some(user.id.clone());
                    resp.payload.token = Some(token.clone());
                    resp.payload.expire_at = Some(expire_at);
                    resp.header.kind = ResponseOk;
                }
                Err(err) => {
                    resp.payload.error = Some(err.into());
                }
            };

        }
        Ok(None) =>  {
            resp.payload.error = Some("account not found".into());
        },
        Err(e) => {
            resp.payload.error = Some(format!("auth failed: {}", e));
        },
    };
    resp
}

pub async fn token_auth(request: AtlasWireMessage<TokenAuthReq>) -> AtlasWireMessage<AuthResp> {
    let token_auth_req = request.payload;
    let mut resp = AtlasWireMessage {
        header: request.header.with_kind(ResponseErr),
        payload: AuthResp {
            ok: false,
            uid: None,
            token: None,
            expire_at: None,
            error: None,
        },
    };
    match validate_token(token_auth_req.token.as_str()).await {
        Ok((uid,expire_at)) => {
            resp.payload.ok = true;
            resp.payload.uid = Some(uid);
            resp.payload.token = Some(token_auth_req.token);
            resp.payload.expire_at = Some(expire_at);
            resp.header.kind = ResponseOk;
        }
        Err(err) => {
            resp.payload.error = Some(err.into());
        }
    }
    resp
}
