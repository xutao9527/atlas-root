use std::sync::Arc;
use tokio::{io, select};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use crate::ws_client::WsClient;

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
            ["t",text] => {
                if let Some(client) = &self.client {
                    client.send_text(text.to_string()).await
                }
            },
            _ => {}
        }
        true
    }
}