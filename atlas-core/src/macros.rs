#[macro_export]
macro_rules! atlas_module_method {
    (
        module $mod_name:ident {
            module_id = $module_id:expr;
            $(
                $method_ty:ident = ($method_id:expr, $req_ty:ty, $resp_ty:ty, $fn_name:path)
            ),* $(,)?
        }
    ) => {
        pub mod $mod_name {
            use super::*;
            use bytes::Bytes;
            use atlas_core::AtlasMethodSpec;
            use atlas_core::net::rpc::packet_request::AtlasRawRequest;
            use atlas_core::net::rpc::packet_response::AtlasRawResponse;
            use atlas_core::net::rpc::router::handle;

            $(
                #[derive(Debug, Copy, Clone, Eq, PartialEq)]
                pub struct $method_ty;

                impl ::atlas_core::AtlasMethodSpec for $method_ty {
                    const MODULE_ID: ::atlas_core::AtlasModuleId = $module_id;
                    const METHOD_ID: u16 = $method_id;
                    type Request = $req_ty;
                    type Response = $resp_ty;
                }
            )*

            pub async fn dispatch(raw: AtlasRawRequest) -> AtlasRawResponse {
                match raw.method {
                    $(
                        $method_ty::WIRE => handle::<$method_ty, _>(raw, $fn_name).await,
                    )*
                    _ => AtlasRawResponse {
                        id: raw.id,
                        slot_index: raw.slot_index,
                        payload: Bytes::new(),
                        error: Some("method not found".into()),
                    },
                }
            }
        }
    };
}

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
            $(
                #[derive(Debug, Copy, Clone, Eq, PartialEq)]
                pub struct $method_ty;

                impl ::atlas_core::AtlasMethodSpec for $method_ty {
                    const MODULE_ID: ::atlas_core::AtlasModuleId = $module_id;
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
            use atlas_core::AtlasMethodSpec;
            use atlas_core::net::rpc::packet_request::AtlasRawRequest;
            use atlas_core::net::rpc::packet_response::AtlasRawResponse;
            use atlas_core::net::rpc::router::handle;

            pub async fn dispatch(raw: AtlasRawRequest) -> AtlasRawResponse {
                match raw.method {
                    $(
                        <$method_ty>::WIRE => handle::<$method_ty, _>(raw, $fn_name).await,
                    )*
                    _ => AtlasRawResponse {
                        id: raw.id,
                        slot_index: raw.slot_index,
                        payload: Bytes::new(),
                        error: Some("method not found".into()),
                    },
                }
            }
        }
    };
}
