use crate::ws_client::WsClient;
use atlas_core::net::rpc::router::AtlasRpcSpec;
use atlas_scheme::proto::auth::rpc::{BasicAuthReq, RegisterReq, TokenAuthReq};
use atlas_scheme::module_method::auth_method::{BasicAuthRpc, RegisterRpc, TokenAuthRpc};
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
                        // let raw_msg = AtlasRawMessage::from_wire_bytes(_resp);
                        // let resp_msg = AtlasWireMessage::<LoginResp>::from_raw(raw_msg.unwrap());
                        println!("{:?}", _resp);
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
            ["api", "reg", account, password,nickname] => {
                if let Some(client) = &self.client {
                    let req = RegisterRpc::build_request(RegisterReq {
                        account: account.to_string(),
                        password: password.to_string(),
                        nickname: nickname.to_string(),
                    }).unwrap();

                    client.send_byte(req.into_wire_bytes()).await;
                }
            },
            ["api","log",account, password] => {
                if let Some(client) = &self.client {
                    let req = BasicAuthRpc::build_request(BasicAuthReq {
                        account: account.to_string(),
                        password: password.to_string(),
                    }).unwrap();
                    println!("Send: {:?}", req);
                    let bytes = req.into_wire_bytes();
                    client.send_byte(bytes).await;
                }
            },
            ["api","auth",token] => {
                if let Some(client) = &self.client {
                    let req = TokenAuthRpc::build_request(TokenAuthReq {
                        token: token.to_string(),
                    }).unwrap();
                    println!("Send: {:?}", req);
                    let bytes = req.into_wire_bytes();
                    client.send_byte(bytes).await;
                }
            }

            _ => {}
        }
        true
    }
}