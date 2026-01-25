use bytes::Bytes;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use atlas_core::net::protocol::frame::{AtlasFrame, AtlasRawFrame};
use atlas_scheme::proto::auth::rpc::AuthResp;

pub struct WsClient{
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    ws_read: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    callback: Arc<dyn Fn(&str) + Send + Sync + 'static>,
}

impl WsClient {

    pub async fn new<F>(ws_server_addr: String,callback: F,) -> WsClient
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        let (ws_stream, _) =
            connect_async(ws_server_addr).await.expect("Failed to connect");
        let (write, read) = ws_stream.split();
        Self {
            ws_write: Arc::new(Mutex::new(write)),
            ws_read: Arc::new(Mutex::new(read)),
            callback: Arc::new(callback),
        }
    }

    pub async fn send_text(&self, text: String) {
        let mut write = self.ws_write.lock().await;
        write.send(Message::Text(text.into())).await.expect("send text failed");
    }

    pub async fn send_byte(&self, buf: Bytes) {
        let mut write = self.ws_write.lock().await;
        write.send(Message::Binary(buf)).await.expect("send byte failed");
    }

    pub async fn run(&mut self,) {
        let ws_read = self.ws_read.clone();
        let callback = self.callback.clone();
        tokio::spawn(async move {
            let mut read = ws_read.lock().await;
            while let Some(msg) =  read.next().await{
                match msg {
                    Ok(Message::Text(text)) => {
                        // println!("Received: {}", text);
                        callback(&text);
                    }
                    Ok(Message::Binary(bin)) => {
                        let result = AtlasRawFrame::from_bytes(bin).unwrap();
                        let result = AtlasFrame::<AuthResp>::from_raw(result);
                        println!("Received: {:?}", result);
                    }
                    Ok(Message::Close(_)) => {
                        println!("Server closed connection");
                        break;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                }
            }
        });
    }
}