use tera::Tera;

mod collect_type;
mod generate_notify;
mod generate_rpc;
mod generate_type;
mod mapping_module_id;
mod mapping_type_rust_to_ts;

pub use collect_type::*;
pub use generate_notify::*;
pub use generate_rpc::*;
pub use generate_type::*;
pub use mapping_module_id::*;
pub use mapping_type_rust_to_ts::*;

pub fn load_tera() -> Tera {
    let glob = format!("{}/templates/**/*", env!("CARGO_MANIFEST_DIR"));
    Tera::new(&glob).expect("load tera templates failed")
}
