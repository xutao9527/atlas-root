use std::fmt;
use std::process::Command;
use ulid::Ulid;
use crate::model::card::{Card, Deck};

#[derive(Debug, Clone)]
pub struct Player {
    /// 玩家全局唯一 ID
    pub id: String,
    /// 玩家昵称
    pub nickname: String,
    /// 玩家当前可用筹码
    pub balance: u64,
    /// 本下注轮（当前 street）中已投入的筹码
    pub street_bet: u64,
    /// 是否仍在牌局中（Fold 后为 false）
    pub is_active: bool,
    // 本下注轮是否已经行动过
    pub has_acted: bool,
    // 是否已经 all-in
    pub is_all_in: bool,
    /// 玩家手牌（底牌），每人 2 张
    pub hole_cards: [Option<Card>; 2],
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

/// 一局德州扑克中的「下注阶段」（也称 Street）
///
/// 每个阶段都会经历一次完整的下注轮：
/// 所有仍在牌局中的玩家都有机会行动，直到下注轮结束。
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Street {
    /// 翻牌前（Pre-Flop）
    /// - 发底牌之后
    /// - 扣完小盲 / 大盲
    /// - 第一轮下注从大盲左手开始
    PreFlop,
    /// 翻牌圈（Flop）
    /// - 公共牌一次性发出 3 张
    /// - 新一轮下注从庄家左手开始
    Flop,
    /// 转牌圈（Turn）
    /// - 公共牌第 4 张
    /// - 新一轮下注从庄家左手开始
    Turn,
    /// 河牌圈（River）
    /// - 公共牌第 5 张
    /// - 最后一轮下注
    River,
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
    /// 动作在当前下注状态下不合法（如非法 check）
    InvalidAction,
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
    /// 当前所处的下注阶段（Street）
    /// - 决定是否需要发公共牌
    /// - 决定下注轮结束后要进入哪里
    pub street: Street,
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
    /// 当前局的牌堆
    pub deck: Deck,
    /// 公共牌（Community Cards），最多 5 张
    pub community_cards: [Option<Card>; 5],
}

impl Table {
    pub fn new() -> Self {
        Self {
            id: Ulid::new().to_string(),
            seats: Default::default(),
            state: TableState::Waiting,
            street: Street::PreFlop,
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
            deck: Deck::new(),
            community_cards: Default::default(),
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

        if self.seats.iter().filter(|s| s.is_some()).count() < 2 {
            return Err(TableError::NotEnoughPlayers);
        }

        self.state = TableState::Preparing;
        // =============== 初始化新的一局 ===============
        // 生成新一局 hand_id
        self.hand_id = Ulid::new().to_string();
        // 初始化 Street
        self.street = Street::PreFlop;
        self.pot = 0;
        self.current_bet = 0;
        // 重置所有玩家的状态
        for p in self.seats.iter_mut().flatten() {
            p.is_active = true;
            p.has_acted = false;
            p.is_all_in = false;
            p.street_bet = 0;
        }
        // 推进庄家按钮
        self.dealer_pos = self.next_occupied_seat(self.dealer_pos).unwrap();
        self.small_blind_pos = self.next_occupied_seat(self.dealer_pos).unwrap();
        self.big_blind_pos   = self.next_occupied_seat(self.small_blind_pos).unwrap();

        // 扣盲注（强制）
        self.post_blind(self.small_blind_pos, self.small_blind_amount);
        self.post_blind(self.big_blind_pos, self.big_blind_amount);

        // 当前下注轮的最高注额 = 大盲
        self.current_bet = self.big_blind_amount;

        // Pre-Flop：大盲视为初始加注者
        self.last_raiser_pos = self.big_blind_pos;

        // Pre-Flop 第一个行动的人（大盲左手）
        self.current_turn = self.next_occupied_seat(self.big_blind_pos).unwrap();

        // 生成并洗牌
        self.deck.shuffle();

        // 给每个玩家发两张底牌
        for p in self.seats.iter_mut().flatten() {
            p.hole_cards[0] = self.deck.deal_one();
            p.hole_cards[1] = self.deck.deal_one();
        }
        // 清空公共牌
        self.community_cards = [None; 5];
        // ===========================================

        // 进入对战阶段
        self.state = TableState::Battling;
        Ok(())
    }

    pub fn act(&mut self, seat: usize, action: PlayerAction) -> Result<(), TableError> {
        if self.state != TableState::Battling {
            return Err(TableError::InvalidState);
        }
        if seat != self.current_turn {
            return Err(TableError::InvalidSeat);
        }

        let player = self.seats[seat].as_mut().unwrap();
        if !player.is_active || player.is_all_in {
            return Err(TableError::InvalidAction);
        }

        // ★ 是否重新打开下注轮（Raise 且有效）
        let mut reopened_betting = false;

        // 先处理动作
        match action {
            PlayerAction::Fold => {
                player.is_active = false;

            }
            PlayerAction::Check  => {
                if player.street_bet != self.current_bet {
                    return Err(TableError::InvalidAction);
                }

            }
            PlayerAction::Call  => {
                let need = self.current_bet - player.street_bet;
                let actual = need.min(player.balance);

                player.balance -= actual;
                player.street_bet += actual;
                self.pot += actual;

                if player.balance == 0 {
                    player.is_all_in = true;
                }


            }
            PlayerAction::Raise(raise_amount) => {
                // 规则：raise_amount 是“在当前 bet 基础上的加注额”
                let target_bet = self.current_bet + raise_amount;
                let need = target_bet - player.street_bet;
                let actual = need.min(player.balance);

                player.balance -= actual;
                player.street_bet += actual;
                self.pot += actual;

                if player.balance == 0 {
                    player.is_all_in = true;
                }

                // 是否真正形成 raise
                if player.street_bet > self.current_bet {
                    // 更新桌面状态
                    self.current_bet = player.street_bet;
                    self.last_raiser_pos = seat;
                    reopened_betting = true;

                }

            }
        }
        player.has_acted = true;
        // ===== 到这里，player 的 &mut 借用已经结束 =====

        // ★ Raise 会让其他人“重新需要回应”
        if reopened_betting {
            for p in self.seats.iter_mut().flatten() {
                if p.is_active && !p.is_all_in {
                    p.has_acted = false;
                }
            }
            // Raise 的人自己已经 acted
            self.seats[seat].as_mut().unwrap().has_acted = true;
        }

        // ★ 检查是否还有人可以行动
        let can_act = self.seats.iter().flatten().any(|p| {
            p.is_active && !p.is_all_in && (p.street_bet != self.current_bet || !p.has_acted)
        });

        if !can_act {
            // ★ 没有人还能 act，直接结束下注轮
            self.end_betting_round();
            return Ok(());
        }

        // ===== 推进行动顺序 =====
        match self.next_occupied_seat(seat) {
            Some(next) => {
                self.current_turn = next;
                // ★ 下注轮结束判断
                if self.is_betting_round_finished() {
                    self.end_betting_round();
                }
            }
            None => {
                // ★ 没有人还能 act，强制结束下注轮
                self.end_betting_round();
            }
        }
        Ok(())
    }

    fn end_betting_round(&mut self) {
        println!("betting round finished: {:?}", self.street);
        // 清空 street 状态
        for p in self.seats.iter_mut().flatten() {
            p.street_bet = 0;
            p.has_acted = false;
        }
        self.current_bet = 0;

        // ★ 核心判断：是否还有人能下注
        let can_bet = self.seats.iter().flatten().any(|p| p.is_active && !p.is_all_in);


        match self.street {
            Street::PreFlop => {
                self.street = Street::Flop;
                // 发三张 Flop 公共牌
                for i in 0..3 {
                    self.community_cards[i] = self.deck.deal_one();
                }
            }
            Street::Flop => {
                self.street = Street::Turn;
                // 发 Turn 第 4 张
                self.community_cards[3] = self.deck.deal_one();
            }
            Street::Turn => {
                self.street = Street::River;
                // 发 River 第 5 张
                self.community_cards[4] = self.deck.deal_one();
            }
            Street::River => {
                self.state = TableState::Concluding;
                println!("hand finished, go to showdown");
                return;
            }
        }

        if !can_bet {
            // ★ 自动推进，直到 River
            self.end_betting_round();
            return;
        }

        // 正常新一轮下注，找到下一个可行动的玩家
        if let Some(pos) = self.next_occupied_seat(self.dealer_pos) {
            self.current_turn = pos;
            self.last_raiser_pos = self.dealer_pos;
        }
    }

    fn is_betting_round_finished(&self) -> bool {
        for p in self.seats.iter().flatten() {
            if !p.is_active {
                continue;
            }
            if p.is_all_in {
                continue;
            }
            // 还没行动过，或者下注没跟平
            if !p.has_acted || p.street_bet != self.current_bet {
                return false;
            }
        }
        true
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
        player.street_bet += actual; // ★★★ 关键
        self.pot += actual;
    }

    /// 从某个座位开始，顺时针查找下一个有玩家的座位
    /// - 用于：
    ///   - 推进庄家按钮
    ///   - 计算盲注位置
    ///   - 推进行动顺序
    fn next_occupied_seat(&self, from: usize) -> Option<usize> {
        let mut i = (from + 1) % self.seats.len();
        let start = i;
        loop {
            if let Some(p) = &self.seats[i] {
                if p.is_active && !p.is_all_in {
                    return Some(i);
                }
            }
            i = (i + 1) % self.seats.len();
            if i == start {
                break;
            }
        }
        None // ★ 没有人还能 act
    }
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        // 其他系统用 ANSI 转义码
        print!("\x1B[2J\x1B[1;1H");
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // \x1B[2J: 清屏, \x1B[1;1H: 光标移动到 (1,1)
        //writeln!(f, "\x1B[2J\x1B[1;1H")?;
        clear_screen();
        writeln!(f, "\n{}", "=".repeat(80))?;
        writeln!(f, "TABLE ID: {} | HAND ID: {}", self.id, self.hand_id)?;
        writeln!(f, "STATE: {:?} | STREET: {:?}", self.state, self.street)?;
        writeln!(f, "TABLE ID: {} | STATE: {:?} | street: {:?}", self.id, self.state, self.street)?;
        writeln!(f, "POT: ${} | CURRENT BET:${} | BLIND_AMOUNT :$({}/{})",
                 self.pot, self.current_bet, self.small_blind_amount,self.big_blind_amount)?;
        writeln!(f, "DEALER_POS: {} | SMALL_BLIND_POS BET: {} | BIG_BLIND_POS BET: {} | CURRENT_TURN_POS: {} | LAST_RAISER_POS: {}",
                 self.dealer_pos, self.small_blind_pos, self.big_blind_pos, self.current_turn, self.last_raiser_pos)?;
        writeln!(f, "{}", "-".repeat(80))?;

        // 显示公共牌
        write!(f, "Community Cards: ")?;
        for card in self.community_cards.iter().flatten() {
            write!(f, "{}  ", card)?;  // 用 {} 而不是 {:?}
        }
        writeln!(f)?;
        writeln!(f, "{}", "-".repeat(80))?;
        for i in 0..10 {
            match &self.seats[i] {
                Some(p) => {
                    let current_turn_mark = if self.current_turn == i { "*" } else { " " };
                    let last_raiser_mark = if self.last_raiser_pos == i { "R" } else { " " };
                    let is_active_mark = if p.is_active { "√" } else { "×" };
                    let is_all_in_mark = if p.is_all_in { " all-in " } else { "        " };
                    let dentity_mark = if self.dealer_pos == i {  "D" }
                    else if self.big_blind_pos == i {  "B" }
                    else if self.small_blind_pos == i {  "S"  }
                    else { " " };

                    let hole_cards_str = p.hole_cards
                        .iter()
                        .map(|c| match c {
                            Some(card) => format!("{}", card),
                            None => "??".to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join("  ");

                    // [*][R][S/B/D][i]  nickname balance Cards
                    writeln!(f,
                             " [{}][{}][{}]  [{}]: {} {} [{:>6}]   [{:>6}] [{:>6}]      |     Cards: {}",
                             current_turn_mark, last_raiser_mark, dentity_mark, i,
                             p.nickname, is_active_mark, is_all_in_mark, p.balance, p.street_bet, hole_cards_str
                    )?;
                }
                None => {
                    writeln!(f,
                             "            [{}]  ( Empty )", i)?;
                }
            }
        }
        writeln!(f, "{}", "=".repeat(80))?;
        write!(f, "{}", "command: [1)show; 2)quit; 3)sit <seat> <balance>; 4)start; 5)act <check> <fold> <call> <raise amount>;]")?;
        Ok(())
    }
}
