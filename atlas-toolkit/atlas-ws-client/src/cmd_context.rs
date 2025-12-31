use crate::ws_client::WsClient;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use tokio::{io, select};
use atlas_auth::rpc::auth_model::LoginReq;
use atlas_auth::rpc::method::AuthMethod;
use atlas_core::net::rpc::packet::{AtlasPacket, AtlasRequest};
use atlas_core::net::rpc::router_spec::AtlasRouterMethod;

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
                    let mut ws_client = WsClient::new(self.ws_server_addr.clone()).await;
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
                    let req = AtlasRequest {
                        id: 0,
                        slot_index: 0 as usize,
                        method: AuthMethod::Login.wire(),
                        payload: LoginReq{
                            account: account.to_string(),
                            password: password.to_string(),
                        },
                    };
                    let raw_req = req.into_raw().unwrap();
                    let packet = AtlasPacket::AtlasRequest(raw_req);
                    let buf = rmp_serde::to_vec(&packet).unwrap();
                    client.send_byte(buf).await;
                }
            }
            _ => {}
        }
        true
    }
}