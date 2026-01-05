#[macro_export]
macro_rules! atlas_method {
    (
        module $mod_name:ident {
            module_id = $module_id:expr;
            $(
                $method_ty:ident = ($method_id:expr, $req_ty:ty, $resp_ty:ty)
            ),* $(,)?
        }
    ) => {
        pub mod $mod_name {
            use super::*;
            use atlas_nut::net::rpc::router::{AtlasMethodSpec, AtlasModuleId};
            $(
                #[derive(Debug, Copy, Clone, Eq, PartialEq)]
                pub struct $method_ty;

                impl AtlasMethodSpec for $method_ty {
                    const MODULE_ID: AtlasModuleId = $module_id;
                    const METHOD_ID: u16 = $method_id;
                    type Request = $req_ty;
                    type Response = $resp_ty;
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! atlas_dispatch {
    (
        module $mod_name:ident {
            $(
                $method_ty:path => $fn_name:path
            ),* $(,)?
        }
    ) => {
        pub mod $mod_name {
            use super::*;
            use bytes::Bytes;
            use atlas_nut::net::rpc::packet_message::AtlasRawMessage;
            use atlas_nut::net::rpc::router::{handle, AtlasMethodSpec};


            pub async fn dispatch(raw: AtlasRawMessage) -> AtlasRawMessage {
                match raw.header.method {
                    $(
                        <$method_ty>::WIRE => handle::<$method_ty, _>(raw, $fn_name).await,
                    )*
                    _ => AtlasRawMessage {
                        header: raw.header,
                        payload: Bytes::new(),
                    },
                }
            }
        }
    };
}
