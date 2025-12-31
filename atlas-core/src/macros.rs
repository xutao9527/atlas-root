#[macro_export]
macro_rules! atlas_methods {
    (
        module $enum_name:ident = $module_id:path {
            $(
                $method:ident = $value:expr
            ),* $(,)?
        }
    ) => {
        #[repr(u16)]
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub enum $enum_name {
            $(
                $method = $value,
            )*
        }

        impl $crate::AtlasRouterMethod for $enum_name {
            const MODULE: $crate::AtlasModuleId = $module_id;

            #[inline(always)]
            fn id(self) -> u16 {
                self as u16
            }
        }
    };
}