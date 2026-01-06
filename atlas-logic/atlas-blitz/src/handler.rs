use crate::auth_mod::{LoginReq, LoginResp, RegisterReq, RegisterResp};
use atlas_core::net::rpc::packet_header::AtlasWireKind::ResponseOk;
use atlas_core::net::rpc::packet_message::AtlasWireMessage;

pub async fn register(request: AtlasWireMessage<RegisterReq>) -> AtlasWireMessage<RegisterResp> {
    AtlasWireMessage {
        header: request.header,
        payload: RegisterResp {
            ok: true,
            error: None,
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
        }
    }
}
