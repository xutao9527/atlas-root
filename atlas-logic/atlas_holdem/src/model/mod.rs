use std::fmt;
use ulid::Ulid;

#[derive(Debug, Clone)]
pub struct Player {
    /// 玩家全局唯一 ID
    pub id: String,
    /// 玩家昵称
    pub nickname: String,
    /// 玩家当前可用筹码
    pub balance: u64,
}

#[derive(Debug)]
pub enum PlayerAction {
    Fold,
    Call,
    Check,
    Raise(u64), // 先留着，不急着实现
}

#[derive(Debug, PartialEq)]
pub enum TableState {
    /// 空闲状态：
    /// - 允许玩家 sit
    /// - 不存在 hand_id
    /// - 不允许下注、行动
    Waiting,
    /// 准备阶段：
    /// - 系统初始化一局游戏
    /// - 推进庄家、扣盲注、设置行动顺序
    /// - 该状态通常是“瞬时状态”
    Preparing,
    /// 对战阶段：
    /// - 玩家按顺序行动（fold / call / raise）
    /// - current_turn 有效
    Battling,
    /// 结算阶段：
    /// - 比牌
    /// - 分配底池
    /// - 更新玩家余额
    Concluding,
}

#[derive(Debug)]
pub enum TableError {
    /// 座位索引非法（超出 0..10）
    InvalidSeat,
    /// 该座位已经有玩家
    SeatOccupied,
    /// 当前桌子状态不允许该操作
    InvalidState,
    /// 坐下的玩家数量不足，无法开始一局
    NotEnoughPlayers,
}

pub struct Table {
    /// 桌子的唯一 ID（生命周期贯穿整个桌子）
    pub id: String,

    /// 桌子上的固定座位数组
    /// - 索引即座位号（0-9）
    /// - None 表示空座
    pub seats: [Option<Player>; 10],
    /// 当前桌子的状态机状态
    pub state: TableState,

    /// 当前这一局（hand）的唯一标识
    /// - 每次 start() 生成
    /// - 用于日志、回放、审计
    pub hand_id: String,

    /// 小盲注金额（桌子级别的固定规则）
    pub small_blind_amount: u64,
    /// 大盲注金额（桌子级别的固定规则）
    pub big_blind_amount: u64,

    /// 当前局已经进入底池的总金额
    pub pot: u64,
    /// 当前下注轮中需要跟注的最大下注额
    /// - Pre-Flop 初始为 big blind
    pub current_bet: u64,

    /// 当前局庄家按钮所在的座位索引
    /// - 每局顺时针移动
    /// - 用于计算盲注和行动顺序
    pub dealer_pos: usize,
    /// 当前局小盲注所在的座位索引
    pub small_blind_pos: usize,
    /// 当前局大盲注所在的座位索引
    pub big_blind_pos: usize,
    /// 当前轮到行动的座位索引
    pub current_turn: usize,
    /// 当前下注轮中，最后一次加注的玩家位置
    /// - 初始为大盲
    /// - 每次 Raise 更新
    pub last_raiser_pos: usize,
}

impl Table {
    pub fn new() -> Self {
        Self {
            id: Ulid::new().to_string(),
            seats: Default::default(),
            state: TableState::Waiting,
            hand_id: String::new(),
            small_blind_amount: 10,
            big_blind_amount: 20,
            pot: 0,
            current_bet: 0,
            dealer_pos: 0,
            small_blind_pos: 0,
            big_blind_pos: 0,
            current_turn: 0,
            last_raiser_pos: 0,
        }
    }

    /// 玩家坐下
    /// 规则：
    /// - 只能在 Waiting 状态下调用
    /// - 玩家必须指定一个空座位
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

    /// 开始新的一局（hand）
    /// 这是一个**系统行为**，不是玩家指令
    pub fn start(&mut self) -> Result<(), TableError> {
        if self.state != TableState::Waiting {
            return Err(TableError::InvalidState);
        }
        let player_count = self.seats.iter().filter(|s| s.is_some()).count();
        if player_count < 2 {
            return Err(TableError::NotEnoughPlayers);
        }

        self.state = TableState::Preparing;
        // ===== 初始化新的一局 =====

        // 生成新一局 hand_id
        self.hand_id = Ulid::new().to_string();
        self.pot = 0;
        self.current_bet = 0;

        // 推进庄家按钮
        self.dealer_pos = self.next_occupied_seat(self.dealer_pos);
        self.small_blind_pos = self.next_occupied_seat(self.dealer_pos);
        self.big_blind_pos   = self.next_occupied_seat(self.small_blind_pos);

        // 扣盲注（强制）
        self.post_blind(self.small_blind_pos, self.small_blind_amount);
        self.post_blind(self.big_blind_pos, self.big_blind_amount);

        // 当前下注轮的最高注额 = 大盲
        self.current_bet = self.big_blind_amount;

        // Pre-Flop：大盲视为初始加注者
        self.last_raiser_pos = self.big_blind_pos;

        // Pre-Flop 第一个行动的人（大盲左手）
        self.current_turn = self.next_occupied_seat(self.big_blind_pos);
        // ======================

        // 进入对战阶段
        self.state = TableState::Battling;
        Ok(())
    }

    pub fn act(&mut self, seat: usize, action: PlayerAction) -> Result<(), TableError> {
        if self.state != TableState::Battling {
            return Err(TableError::InvalidState);
        }
        if seat != self.current_turn {
            return Err(TableError::InvalidSeat); // 之后可以细化
        }
        match action {
            PlayerAction::Fold => {
                self.current_turn = self.next_occupied_seat(seat);
            }
            PlayerAction::Call | PlayerAction::Check  => {
                self.current_turn = self.next_occupied_seat(seat);
            }
            PlayerAction::Raise(_) => {
                self.last_raiser_pos = seat;
                self.current_turn = self.next_occupied_seat(seat);
            }
        }
        // ★ 下注轮结束判断
        if self.current_turn == self.last_raiser_pos {
            self.end_betting_round();
        }

        Ok(())
    }

    fn end_betting_round(&mut self) {
        println!("betting round finished");

        // 现在先不发牌、不比牌
        // 你可以先简单打印 / 切状态
    }


    /// 扣除指定座位玩家的盲注
    ///
    /// - 这是系统行为，不是玩家行为
    /// - 扣除金额直接进入底池
    /// - 当前实现中：
    ///   - 不区分是否 all-in
    ///   - 若余额不足，则扣到 0 为止
    fn post_blind(&mut self, seat: usize, amount: u64) {
        // 该座位一定有玩家（由 start() 的逻辑保证）
        let player = self.seats[seat].as_mut().unwrap();
        // 实际扣除金额（余额不足时等于余额）
        let actual = amount.min(player.balance);
        // 更新玩家余额和底池
        player.balance -= actual;
        self.pot += actual;
    }

    /// 从某个座位开始，顺时针查找下一个有玩家的座位
    /// - 用于：
    ///   - 推进庄家按钮
    ///   - 计算盲注位置
    ///   - 推进行动顺序
    fn next_occupied_seat(&self, from: usize) -> usize {
        let mut i = (from + 1) % self.seats.len();
        loop {
            if self.seats[i].is_some() {
                return i;
            }
            i = (i + 1) % self.seats.len();
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // \x1B[2J: 清屏, \x1B[1;1H: 光标移动到 (1,1)
        writeln!(f, "\x1B[2J\x1B[1;1H")?;
        writeln!(f, "\n{}", "=".repeat(60))?;
        writeln!(f, "TABLE ID: {} | STATE: {:?}", self.id, self.state)?;
        writeln!(f, "POT: ${} | CURRENT BET:${} | BLIND_AMOUNT :$({}/{})",
                 self.pot, self.current_bet, self.small_blind_amount,self.big_blind_amount)?;
        writeln!(f, "DEALER_POS: {} | SMALL_BLIND_POS BET: {} | BIG_BLIND_POS BET: {} | CURRENT_TURN_POS: {} | LAST_RAISER_POS: {}",
                 self.dealer_pos, self.small_blind_pos, self.big_blind_pos, self.current_turn, self.last_raiser_pos)?;
        writeln!(f, "{}", "-".repeat(60))?;

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
        writeln!(f, "{}", "=".repeat(60))?;

        write!(f, "{}", "command: [show; quit; sit <seat> <balance>; start]")?;
        Ok(())
    }
}

