use atlas_nut::net::rpc::packet_message::AtlasWireMessage;
use crate::dto::{LoginReq, LoginResp, RegisterReq, RegisterResp};


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
        header: request.header,
        payload: LoginResp {
            ok: true,
            token: Option::from(token),
            error: None,
        }
    }
}
