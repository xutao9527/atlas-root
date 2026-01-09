use std::sync::{Arc, OnceLock};
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::{mpsc, Mutex};
use atlas_holdem::model::Table;

static G_TABLE: OnceLock<Arc<Mutex<Table>>> = OnceLock::new();
fn get_table() -> &'static Mutex<Table> {
    // get_or_init 确保只初始化一次
    G_TABLE.get_or_init(|| Arc::new(Mutex::new(Table::new())))
}

#[tokio::main]
async fn main() {
    run_cmd().await;
}

async fn run_cmd(){
    // 发送命令行
    let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel::<String>();
    tokio::spawn(async move {
        let mut stdin = BufReader::new(io::stdin()).lines();

        while let Ok(Some(line)) = stdin.next_line().await {
            let _ = cmd_tx.send(line.clone());
            if line.trim() == "q" {
                break;
            }
        }
    });
    while let Some(cmd) = cmd_rx.recv().await {
        if !handle_cmd(cmd).await {

            break
        }
    }
}

async fn handle_cmd(cmd: String) -> bool {
    let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
    let command = parts.as_slice();
    let table = get_table().lock().await;
    match command {
        ["q"] => {
            return false;
        },
        ["show"] => {
            println!("{}", *table);
        }
        _ => {

        }
    }
    true
}
