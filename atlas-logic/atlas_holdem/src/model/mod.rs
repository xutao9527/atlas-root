use std::fmt;
use ulid::Ulid;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,
    pub nickname: String,
    pub balance: u64,
}

#[derive(Debug, PartialEq)]
pub enum TableState {
    Waiting,
    Preparing,
    Battling,
    Concluding,
}

#[derive(Debug)]
pub enum TableError {
    InvalidSeat,
    SeatOccupied,
    InvalidState,
}

pub struct Table {
    pub id: String,
    pub seats: [Option<Player>; 10],
    pub state: TableState,
    pub pot: u64,
    pub current_bet: u64,
}

impl Table {
    pub fn new() -> Self {
        Self {
            id: Ulid::new().to_string(),
            seats: Default::default(),
            state: TableState::Waiting,
            pot: 0,
            current_bet: 0,
        }
    }

    pub fn sit(&mut self,  seat: usize, player: Player) -> Result<(), TableError> {
        if seat >= self.seats.len() {
            return Err(TableError::InvalidSeat);
        }
        if self.state != TableState::Waiting {
            return Err(TableError::InvalidState);
        }
        if self.seats[seat].is_some() {
            return Err(TableError::SeatOccupied);
        }
        self.seats[seat] = Some(player);
        Ok(())
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // \x1B[2J: 清屏, \x1B[1;1H: 光标移动到 (1,1)
        writeln!(f, "\x1B[2J\x1B[1;1H")?;
        writeln!(f, "\n{}", "=".repeat(40))?;
        writeln!(f, "TABLE ID: {} | STATE: {:?}", self.id, self.state)?;
        writeln!(f, "POT: $ {} | CURRENT BET: $ {}", self.pot, self.current_bet)?;
        writeln!(f, "{}", "-".repeat(40))?;

        for i in 0..10 {
            match &self.seats[i] {
                Some(p) => {
                    writeln!(f, "  [{}] {}: ${}", i, p.nickname, p.balance)?;
                }
                None => {
                    writeln!(f, "  [{}] (Empty Seat)", i)?;
                }
            }
        }
        writeln!(f, "{}", "=".repeat(40))?;

        write!(f, "{}", "command: [show; quit; sit<seat, name, balance>]")?;
        Ok(())
    }
}

