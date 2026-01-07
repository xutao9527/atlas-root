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

    let id = Ulid::new();

    // 1️⃣ 官方标准：Crockford Base32（26 位，最常用）
    println!("ULID string (base32): {}", id.to_string());

    // 2️⃣ 原始字节（[u8; 16]）
    let bytes = id.to_bytes();
    println!("ULID bytes ([u8;16]): {:?}", bytes);



    // 4️⃣ 时间戳部分（毫秒）
    println!("ULID timestamp (ms): {}", id.timestamp_ms());

    // 5️⃣ 从 bytes 还原 ULID
    let id_from_bytes = Ulid::from_bytes(bytes);
    println!("From bytes -> ULID: {}", id_from_bytes);




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
                    message: None,
                }
            }
        }
        Err(e) => {
            log::error!("register error: {:?}", e);
            AtlasWireMessage {
                header: request.header.with_kind(ResponseErr),
                payload: RegisterResp {
                    ok: false,
                    message: None,
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
