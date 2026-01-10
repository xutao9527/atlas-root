use atlas_holdem::model::table::{Player, PlayerAction, Table};
use std::sync::{Arc, OnceLock};
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::{mpsc, Mutex};
use ulid::Ulid;

static G_TABLE: OnceLock<Arc<Mutex<Table>>> = OnceLock::new();
fn get_table() -> &'static Mutex<Table> {
    // get_or_init 确保只初始化一次
    G_TABLE.get_or_init(|| Arc::new(Mutex::new(Table::new_six(10,20))))
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
    handle_cmd("sit 0 2000".into()).await;
    handle_cmd("sit 1 600".into()).await;
    handle_cmd("sit 2 2000".into()).await;
    handle_cmd("sit 3 2000".into()).await;
    handle_cmd("sit 4 500".into()).await;
    handle_cmd("sit 5 2000".into()).await;
    handle_cmd("sit 6 2000".into()).await;
    // handle_cmd("sit 7 1000".into()).await;
    // handle_cmd("sit 8 1200".into()).await;
    // handle_cmd("sit 9 2000".into()).await;
    handle_cmd("start".into()).await;
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
                street_bet: 0,
                is_active: false,
                has_acted: false,
                is_all_in: false,
                hole_cards: [None;2],
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
        ["act", "fold"] => {
            act_and_show(&mut table, PlayerAction::Fold);
        }
        ["act", "call"] => {
            act_and_show(&mut table, PlayerAction::Call);
        }
        ["act", "check"] => {
            act_and_show(&mut table, PlayerAction::Check);
        }
        ["act", "raise", amount] => {
            let amount: u64 = match amount.parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("invalid raise amount");
                    return true;
                }
            };
            act_and_show(&mut table, PlayerAction::Raise(amount));
        }
        _ => {
            println!("unknown command: {:?}", command);
        }
    }
    true
}

fn act_and_show(table: &mut Table, action: PlayerAction) {
    let seat = table.current_turn;
    match table.act(seat, action) {
        Ok(_) => println!("{}", *table),
        Err(e) => println!("act failed: {:?}", e),
    }
}