use atlas_holdem::model::player::{Player, PlayerAction};
use atlas_holdem::model::table::Table;
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

static CMD_TX: OnceLock<mpsc::UnboundedSender<String>> = OnceLock::new();

async fn run_cmd(){
    // 发送命令行
    let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel::<String>();

    let cmd_tx_clone1 = cmd_tx.clone();
    let cmd_tx_clone2 = cmd_tx.clone();
    let cmd_tx_clone3 = cmd_tx.clone();

    CMD_TX.set(cmd_tx_clone1).expect("CMD_TX already set");

    tokio::spawn(async move {
        let mut stdin = BufReader::new(io::stdin()).lines();
        while let Ok(Some(line)) = stdin.next_line().await {
            let _ = cmd_tx_clone2.send(line.clone());
            if line.trim() == "quit" {
                break;
            }
        }
    });

    tokio::spawn(async move {
        //tokio::time::sleep(Duration::from_millis(500)).await;
        let _ = cmd_tx_clone3.send("sit 0 2000".into());
        let _ = cmd_tx_clone3.send("sit 1 2000".into());
        let _ = cmd_tx_clone3.send("sit 2 2000".into());
        let _ = cmd_tx_clone3.send("sit 3 2000".into());
        let _ = cmd_tx_clone3.send("sit 4 2000".into());
        let _ = cmd_tx_clone3.send("sit 5 2000".into());
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
                hand_cards: [None; 2],
                cards_str: "".to_string(),
                win: false,
                cards_rank: None,
                is_active: false,
                has_acted: false,
                is_all_in: false,
                street_bet: 0,
                total_bet: 0,
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
        ["quick"] => {
            quick_battling().await;
            println!("{}", *table);
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
        ["act", "bet", amount] => {
            let amount: u64 = match amount.parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("invalid raise amount");
                    return true;
                }
            };
            act_and_show(&mut table, PlayerAction::Bet(amount));
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


async fn quick_battling() {
    if let Some(cmd_tx )= CMD_TX.get(){
        let _ = cmd_tx.send("start".into());
        // PreFlop
        let _ = cmd_tx.send("act call".into());
        let _ = cmd_tx.send("act call".into());
        let _ = cmd_tx.send("act call".into());
        let _ = cmd_tx.send("act call".into());
        let _ = cmd_tx.send("act call".into());
        let _ = cmd_tx.send("act call".into());
        // Flop
        let _ = cmd_tx.send("act check".into());
        let _ = cmd_tx.send("act check".into());
        let _ = cmd_tx.send("act check".into());
        let _ = cmd_tx.send("act check".into());
        let _ = cmd_tx.send("act check".into());
        let _ = cmd_tx.send("act check".into());
        // turn
        let _ = cmd_tx.send("act check".into());
        let _ = cmd_tx.send("act check".into());
        let _ = cmd_tx.send("act check".into());
        let _ = cmd_tx.send("act check".into());
        let _ = cmd_tx.send("act check".into());
        let _ = cmd_tx.send("act check".into());
        // River
        let _ = cmd_tx.send("act check".into());
        let _ = cmd_tx.send("act check".into());
        let _ = cmd_tx.send("act check".into());
        let _ = cmd_tx.send("act check".into());
        let _ = cmd_tx.send("act check".into());
        let _ = cmd_tx.send("act check".into());
    }
}