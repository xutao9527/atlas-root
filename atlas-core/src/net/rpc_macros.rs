#[macro_export]
macro_rules! atlas_rpc_module {
    (
        module $mod_name:ident {
            ModuleId = $module_id:expr;
            $(
                $method_ty:ident = ($method_id:expr, $req_ty:ty, $resp_ty:ty)
            ),* $(,)?
        }
    ) => {
        pub mod $mod_name {
            use super::*;

            use atlas_core::net::core::{AtlasModuleId, AtlasRpcSpec};

            // ===== 编译期校验：method_id 不能重复 =====
            #[allow(non_camel_case_types)]
            enum AtlasRpcMethodIdCheck {
                $(
                    $method_ty = $method_id,
                )*
            }

            $(
                #[derive(Debug, Copy, Clone, Eq, PartialEq)]
                pub struct $method_ty;
                impl AtlasRpcSpec for $method_ty {
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
macro_rules! atlas_rpc_dispatch {
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
            use atlas_core::net::core::{handle_rpc, AtlasRpcSpec};
            use atlas_core::net::protocol::frame::AtlasRawFrame;


            pub async fn dispatch(raw: AtlasRawFrame) -> AtlasRawFrame {
                match raw.header.op_code {
                    $(
                        <$method_ty>::OP_CODE => handle_rpc::<$method_ty, _>(raw, $fn_name).await,
                    )*
                    _ => AtlasRawFrame {
                        header: raw.header,
                        body: Bytes::new(),
                    },
                }
            }
        }
    };
}
