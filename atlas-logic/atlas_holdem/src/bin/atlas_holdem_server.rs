use atlas_holdem::model::{Player, Table};
use std::sync::{Arc, OnceLock};
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::{mpsc, Mutex};
use ulid::Ulid;

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
            if line.trim() == "quit" {
                break;
            }
        }
    });
    handle_cmd("show".into()).await;
    while let Some(cmd) = cmd_rx.recv().await {
        if !handle_cmd(cmd).await {
            break
        }
    }

}

async fn handle_cmd(cmd: String) -> bool {
    let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
    let command = parts.as_slice();
    let mut table = get_table().lock().await;
    match command {
        ["quit"] => {
            return false;
        },
        ["show"] => {
            println!("{}", *table);
        }
        ["sit", seat, balance] => {
            let seat: usize = match seat.parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("invalid seat");
                    return true;
                }
            };
            let balance: u64 = match balance.parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("invalid balance");
                    return true;
                }
            };

            let player = Player {
                id: Ulid::new().to_string(),
                nickname: format!("player00{}", seat),
                balance,
            };

            match table.sit(seat, player) {
                Ok(_) => {
                    println!("{}", *table);
                }
                Err(e) => {
                    println!("sit failed: {:?}", e);
                }
            }
        }
        ["start"] => {
            match table.start() {
                Ok(_) => {
                    println!("{}", *table);
                }
                Err(e) => {
                    println!("start failed: {:?}", e);
                }
            }
        }
        _ => {
            println!("unknown command: {:?}", command);
        }
    }
    true
}
