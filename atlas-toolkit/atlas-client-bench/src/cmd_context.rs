use crate::ws_client::WsClient;
use atlas_nut::net::rpc::packet_header::{AtlasWireHeader, AtlasWireKind};
use atlas_nut::net::rpc::packet_message::{AtlasRawMessage, AtlasWireMessage};
use atlas_nut::net::rpc::router::AtlasMethodSpec;
use atlas_scheme::dto::auth_model::{LoginReq, LoginResp};
use atlas_scheme::module_method::auth_method::Login;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use tokio::{io, select};

pub struct CmdContext {
    pub ws_server_addr: String,
    pub cmd_rx: mpsc::UnboundedReceiver<String>,
    pub cmd_tx: mpsc::UnboundedSender<String>,
    pub client: Option<WsClient>,
}
impl CmdContext {
    pub fn new(ws_server_addr: String) -> Self {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<String>();
        Self {
            ws_server_addr,
            cmd_rx,
            cmd_tx,
            client: None,
        }
    }

    fn send_cmd_task(&mut self) {
        let cmd_tx = self.cmd_tx.clone();
        tokio::spawn(async move {
            let mut stdin = BufReader::new(io::stdin()).lines();
            loop {
                let line = stdin.next_line().await;
                match line {
                    Ok(Some(line)) => {
                        if line.trim() == "q" {
                            let _ = cmd_tx.send(line);
                            break;
                        } else {
                            let _ = cmd_tx.send(line);
                        }
                    }
                    _ => break,
                }
            }
        });

    }

    pub async fn run(&mut self) {
        self.send_cmd_task();
        let _ = self.cmd_tx.send("c".to_string());
        loop {
            select! {
                Some(cmd) = self.cmd_rx.recv() => {
                    let keep_running =  self.handle_cmd(cmd).await;
                      if !keep_running {
                        break;
                    }
                }
            }
        }
    }

    async fn handle_cmd(&mut self, cmd: String) -> bool {
        let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
        let command = parts.as_slice();
        match command {
            ["q"] => {
                return false;
            },
            ["c"] => {
                if self.client.is_none() {
                    let mut ws_client = WsClient::new(self.ws_server_addr.clone(),|_resp|{
                        let raw_msg = AtlasRawMessage::from_wire_bytes(_resp);
                        let resp_msg = AtlasWireMessage::<LoginResp>::from_raw(raw_msg.unwrap());
                        println!("{:?}", resp_msg);
                    }).await;
                    ws_client.run().await;
                    self.client = Some(ws_client);
                }
            },
            ["text",text] => {
                if let Some(client) = &self.client {
                    client.send_text(text.to_string()).await
                }
            },
            ["api","login",account, password] => {
                if let Some(client) = &self.client {
                    let request = AtlasWireMessage {
                        header: AtlasWireHeader {
                            id: 1,
                            slot_index: 1,
                            method: Login::WIRE,
                            kind: AtlasWireKind::Request,
                        },
                        payload: LoginReq {
                            account: account.to_string(),
                            password: password.to_string(),
                        },
                    };

                    let request_bytes = request.into_raw().unwrap().into_wire_bytes();
                    client.send_byte(request_bytes).await;
                }
            }
            _ => {}
        }
        true
    }
}