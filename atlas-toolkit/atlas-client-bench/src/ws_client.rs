use atlas_core::net::rpc::packet::AtlasPacket;
use atlas_core::net::rpc::packet_response::AtlasRawResponse;
use bytes::Bytes;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

pub struct WsClient{
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    ws_read: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    callback: Arc<dyn Fn(AtlasRawResponse) + Send + Sync + 'static>,
}

impl WsClient {

    pub async fn new<F>(ws_server_addr: String,callback: F,) -> WsClient
    where
        F: Fn(AtlasRawResponse) + Send + Sync + 'static,
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

    pub async fn send_byte(&self, buf: Vec<u8>) {
        let mut write = self.ws_write.lock().await;
        write.send(Message::Binary(Bytes::from(buf))).await.expect("send byte failed");
    }

    pub async fn run(&mut self,) {
        let ws_read = self.ws_read.clone();
        let callback = self.callback.clone();
        tokio::spawn(async move {
            let mut read = ws_read.lock().await;
            while let Some(msg) =  read.next().await{
                match msg {
                    Ok(Message::Text(text)) => {
                        println!("Received: {}", text);
                    }
                    Ok(Message::Binary(bin)) => {
                        let packet: AtlasPacket =  rmp_serde::from_slice(&bin).expect("REASON");
                        match packet {
                            AtlasPacket::AtlasRequest(req) => {
                                println!("ws client Received : {:?}", req);
                            }
                            AtlasPacket::AtlasResponse(resp) => {
                                // let resp = AtlasWireResponse::<LoginResp>::from_raw(resp);
                                // println!("ws client Received : {:?}", resp);
                                callback(resp);
                            }
                        }

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