use crate::context::get_db;
use crate::context::token_manager::{store_token, validate_token};
use atlas_scheme::model::atlas_user;
use atlas_scheme::model::sea_orm_active_enums::UserType;
use atlas_scheme::proto::auth::rpc::{
    AuthResp, BasicAuthReq, RegisterReq, RegisterResp, TokenAuthReq,
};
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel};
use tracing::log;
use ulid::Ulid;
use atlas_core::net::core::{global_notifier, AtlasNotifyBuildExt, NotifierExt};
use atlas_core::net::core::reg_node::AtlasRegNodeId;
use atlas_core::net::protocol::frame::AtlasFrame;
use atlas_core::net::protocol::frame_body_notify::{AtlasNotifyTarget};
use atlas_core::net::protocol::frame_body_rpc::{AtlasRpcBody, AtlasWireError};
use atlas_scheme::proto::auth::notify::user_update_notify::UserUpdateNotify;

pub async fn register(req: AtlasFrame<RegisterReq>) -> AtlasRpcBody<RegisterResp> {
    let model = atlas_user::Model {
        id: Ulid::new().to_string(),
        user_type: Some(UserType::Normal),
        account: req.body.account,
        password: req.body.password,
        name: req.body.nickname,
        balance: Default::default(),
        avatar: None,
        created_at: chrono::Utc::now(),
        updated_at: None,
    };
    let active_model = model.into_active_model();
    let res = active_model.insert(get_db()).await;
    match res {
        Ok(_) => AtlasRpcBody::Ok(RegisterResp {
            ok: true,
            message: Some("register success".into()),
        }),
        Err(e) => {
            log::error!("register error: {:?}", e);
            AtlasRpcBody::Err(AtlasWireError {
                code: 500,
                message: format!("register failed: {}", e),
                data: None,
            })
        }
    }
}

pub async fn basic_auth(req: AtlasFrame<BasicAuthReq>) -> AtlasRpcBody<AuthResp> {
    let result = atlas_user::Entity::find()
        .filter(atlas_user::Column::Account.eq(req.body.account))
        .one(get_db())
        .await;

    match result {
        Ok(Some(user)) => {
            if user.password != req.body.password {
                return AtlasRpcBody::Err(AtlasWireError {
                    code: 401,
                    message: "Invalid password".into(),
                    data: None,
                });
            }
            let token = Ulid::new().to_string();
            match store_token(token.as_str(), user.id.as_str()).await {
                Ok(expire_at) => {
                    if let Some(notifier) = global_notifier() {
                        let notify_msg  = UserUpdateNotify {
                            id: "".to_string(),
                            account: "".to_string(),
                            name: "".to_string(),
                            balance: Default::default(),
                            avatar: None,
                        }.build_notify(vec![
                            AtlasNotifyTarget::Broadcast,
                        ]);
                        notifier.notify(
                            &AtlasRegNodeId::GateNode(1),
                            notify_msg,
                        );
                    }
                    AtlasRpcBody::Ok(AuthResp {
                        ok: true,
                        uid: Some(user.id),
                        token: Some(token),
                        expire_at: Some(expire_at),
                    })
                },
                Err(err) => AtlasRpcBody::Err(AtlasWireError {
                    code: 500,
                    message: err.to_string(),
                    data: None,
                }),
            }
        }
        Ok(None) => AtlasRpcBody::Err(AtlasWireError {
            code: 404,
            message: "account not found".into(),
            data: None,
        }),
        Err(e) => AtlasRpcBody::Err(AtlasWireError {
            code: 500,
            message: format!("auth failed: {}", e),
            data: None,
        }),
    }
}

pub async fn token_auth(req: AtlasFrame<TokenAuthReq>) -> AtlasRpcBody<AuthResp> {
    match validate_token(req.body.token.as_str()).await {
        Ok((uid, expire_at)) => AtlasRpcBody::Ok(AuthResp {
            ok: true,
            uid: Some(uid),
            token: Some(req.body.token),
            expire_at: Some(expire_at),
        }),
        Err(err) => AtlasRpcBody::Err(AtlasWireError {
            code: 401,
            message: err.to_string(),
            data: None,
        }),
    }
}
