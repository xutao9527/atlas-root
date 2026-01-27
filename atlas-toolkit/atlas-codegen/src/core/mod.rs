mod generate_rpc;
mod collect_rpc;
mod type_mapping_rust_to_ts;
mod collect_notify;
mod collect_type;
mod generate_type;

use tera::Tera;
pub use generate_rpc::*;
pub use collect_rpc::*;
pub use type_mapping_rust_to_ts::*;
pub use collect_notify::*;
pub use collect_type::*;
pub use generate_type::*;


pub fn load_tera() -> Tera {
    let glob = format!(
        "{}/templates/**/*",
        env!("CARGO_MANIFEST_DIR")
    );
    Tera::new(&glob).expect("load tera templates failed")
}