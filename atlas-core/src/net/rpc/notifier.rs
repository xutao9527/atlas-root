use crate::net::rpc::packet_message::AtlasRawMessage;

pub trait Notifier: Send + Sync {
    fn notify(&self, logical_id: &str, msg: AtlasRawMessage) -> bool;
}