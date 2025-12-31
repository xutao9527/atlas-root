mod cmd_context;
mod ws_client;

use crate::cmd_context::CmdContext;

#[tokio::main]
async fn main() {
    let ws_server_addr = "ws://127.0.0.1:8080/ws".to_string();

    let mut cmd_context = CmdContext::new(ws_server_addr);

    cmd_context.run().await;
}
