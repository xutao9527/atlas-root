use atlas_core::net::rpc::packet_header::AtlasWireKind::ResponseOk;
use atlas_core::net::rpc::packet_message::AtlasWireMessage;
use atlas_scheme::dto::auth_model::{LoginReq, LoginResp, RegisterReq, RegisterResp};

pub async fn register(request: AtlasWireMessage<RegisterReq>) -> AtlasWireMessage<RegisterResp> {
    AtlasWireMessage {
        header: request.header.with_kind(ResponseOk),
        payload: RegisterResp {
            ok: true,
            message: None,
        },
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
