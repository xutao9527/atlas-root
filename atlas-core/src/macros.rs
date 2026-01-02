#[macro_export]
macro_rules! atlas_methods {
    (
        module $mod_name:ident {
            module_id = $module_id:expr;
            $(
                $method_ty:ident = $method_id:expr
            ),* $(,)?
        }
    ) => {
        pub mod $mod_name {
            use super::*;

            $(
                #[derive(Debug, Copy, Clone, Eq, PartialEq)]
                pub struct $method_ty;

                impl $crate::AtlasRouterMethod for $method_ty {
                    const MODULE_ID: $crate::AtlasModuleId = $module_id;
                    const METHOD_ID: u16 = $method_id;
                }
            )*
        }
    };
}
