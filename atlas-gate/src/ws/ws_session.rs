use axum::extract::ws::{Message, Utf8Bytes};
use bytes::Bytes;
use tokio::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub struct WsSession {
    pub uid: Option<String>,
    pub token: Option<String>,
    pub expire_at: Option<u64>, // unix seconds
    pub msg_tx: Sender<Message>,
}

impl WsSession {
    pub fn new(msg_tx: Sender<Message>) -> Self {
        Self {
            uid: None,
            token: None,
            expire_at: None,
            msg_tx,
        }
    }

    pub fn is_authed(&self) -> bool {
        self.uid.is_some()
    }

    pub async fn send_binary(&self, bin: Bytes) {
        let _ = self
            .msg_tx
            .send(Message::Binary(bin))
            .await;
    }

    pub async fn send_text_bytes(&self, text: Utf8Bytes) {
        let _ = self
            .msg_tx
            .send(Message::Text(text))
            .await;
    }

    pub async fn _send_text<S: Into<String>>(&self, text: S) {
        let _ = self
            .msg_tx
            .send(Message::Text(Utf8Bytes::from(text.into())))
            .await;
    }
}
