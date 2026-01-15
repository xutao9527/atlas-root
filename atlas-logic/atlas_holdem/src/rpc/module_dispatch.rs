use crate::rpc::handler::table_handler;
use atlas_core::atlas_rpc_dispatch;
use atlas_scheme::module_method::holdem_method;

atlas_rpc_dispatch! {
    module holdem_bind {
        holdem_method::GetTableRpc => table_handler::get_table,
    }
}


pub mod holdem_bind1 {
    use super::*;
    use bytes::Bytes;
    use atlas_core::net::rpc::packet_message::AtlasRawMessage;
    use atlas_core::net::rpc::router::{handle, AtlasRpcSpec};


    pub async fn dispatch(raw: AtlasRawMessage) -> AtlasRawMessage {
        match raw.header.method {
            <holdem_method::GetTableRpc>::WIRE => handle::<holdem_method::GetTableRpc, _>(raw, table_handler::get_table).await,
            _ => AtlasRawMessage {
                header: raw.header,
                payload: Bytes::new(),
            },
        }
    }
}