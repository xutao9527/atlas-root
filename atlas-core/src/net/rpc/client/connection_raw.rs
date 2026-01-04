use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use bytes::Bytes;
use tokio::sync::{mpsc, Mutex, Notify};
use crate::net::rpc::client::pending::PendingTable;

pub struct AtlasRawConnection {
    addr: String,
    channel_writer: Mutex<mpsc::Sender<Bytes>>,
    pending: Arc<PendingTable<Bytes>>,
    notify_connected: Arc<Notify>,
    notify_disconnected: Arc<Notify>,
    connected: Arc<AtomicBool>,
}
