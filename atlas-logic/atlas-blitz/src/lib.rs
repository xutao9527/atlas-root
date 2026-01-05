pub mod handler;
pub mod dto;

pub mod auth_mod {
  
    use atlas_nut::net::rpc::packet_message::{AtlasRawMessage};
    use atlas_nut::net::rpc::router::{AtlasMethodSpec, AtlasModuleId, handle};
    use bytes::Bytes;
    use serde::{Deserialize, Serialize};
    
    use crate::handler::{login, register};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct RegisterReq {
        pub account: String,
        pub password: String,
    }
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct RegisterResp {
        pub ok: bool,
        pub error: Option<String>,
    }


    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct LoginReq {
        pub account: String,
        pub password: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct LoginResp {
        pub ok: bool,
        pub token: Option<String>,
        pub error: Option<String>,
    }


    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct Register;

    impl AtlasMethodSpec for Register {
        const MODULE_ID: AtlasModuleId = AtlasModuleId::Auth;
        const METHOD_ID: u16 = 1;
        type Request = RegisterReq;
        type Response = RegisterResp;
    }
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct Login;

    impl AtlasMethodSpec for Login {
        const MODULE_ID: AtlasModuleId = AtlasModuleId::Auth;
        const METHOD_ID: u16 = 2;
        type Request = LoginReq;
        type Response = LoginResp;
    }

    pub async fn dispatch(raw: AtlasRawMessage) -> AtlasRawMessage {
        match raw.header.method {
            <Register>::WIRE => handle::<Register, _>(raw, register).await,
            <Login>::WIRE => handle::<Login, _>(raw, login).await,
            _ => AtlasRawMessage {
                header: raw.header,
                payload: Bytes::new(),

            },
        }
    }
}