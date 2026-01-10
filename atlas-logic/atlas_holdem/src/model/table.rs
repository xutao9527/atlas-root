use crate::model::card::{Card, Deck};
use crate::model::player::{Player, PlayerAction};

#[derive(Debug, PartialEq)]
pub enum TableState {
    Waiting,                                    // 空闲状态：
    Preparing,                                  // 准备阶段：
    Battling,                                   // 对战阶段：
    Concluding,                                 // 结算阶段：
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TableStreet {
    PreFlop,                                    // 翻牌前（Pre-Flop）
    Flop,                                       // 翻牌圈（Flop）
    Turn,                                       // 转牌圈（Turn）
    River,                                      // 河牌圈（River）
}

#[derive(Debug)]
pub enum TableError {
    InvalidSeat,                                // 座位索引非法（超出 0..10）
    SeatOccupied,                               // 该座位已经有玩家
    InvalidState,                               // 当前桌子状态不允许该操作
    NotEnoughPlayers,                           // 坐下的玩家数量不足，无法开始一局
    InvalidAction,                              // 动作在当前下注状态下不合法（如非法 check）
}

pub struct Table {
    pub id: String,                             // 桌子的唯一 ID（生命周期贯穿整个桌子）
    pub seats: Vec<Option<Player>>,             // 桌子上的固定座位数组
    pub state: TableState,                      // 当前桌子的状态机状态
    pub street: TableStreet,                    // 当前所处的下注阶段（Street）
    pub hand_id: String,                        // 当前这一局（hand）的唯一标识
    pub small_blind_amount: u64,                // 小盲注金额（桌子级别的固定规则）
    pub big_blind_amount: u64,                  // 大盲注金额（桌子级别的固定规则）
    pub pot: u64,                               // 当前局已经进入底池的总金额
    pub current_bet: u64,                       // 当前下注轮中需要跟注的最大下注额
    pub dealer_pos: usize,                      // 当前局庄家按钮所在的座位索引
    pub small_blind_pos: usize,                 // 当前局小盲注所在的座位索引
    pub big_blind_pos: usize,                   // 当前局大盲注所在的座位索引
    pub current_turn: usize,                    // 当前轮到行动的座位索引
    pub last_raiser_pos: usize,                 // 当前下注轮中，最后一次加注的玩家位置
    pub deck: Deck,                             // 当前局的牌堆
    pub community_cards: [Option<Card>; 5],     // 公共牌（Community Cards），最多 5 张
}

impl Table {




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
            TableStreet::PreFlop => {
                self.street = TableStreet::Flop;
                // 发三张 Flop 公共牌
                for i in 0..3 {
                    self.community_cards[i] = self.deck.deal_one();
                }
            }
            TableStreet::Flop => {
                self.street = TableStreet::Turn;
                // 发 Turn 第 4 张
                self.community_cards[3] = self.deck.deal_one();
            }
            TableStreet::Turn => {
                self.street = TableStreet::River;
                // 发 River 第 5 张
                self.community_cards[4] = self.deck.deal_one();
            }
            TableStreet::River => {
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

    /// 从某个座位开始，顺时针查找下一个有玩家的座位
    /// - 用于：
    ///   - 推进庄家按钮
    ///   - 计算盲注位置
    ///   - 推进行动顺序
    pub fn next_occupied_seat(&self, from: usize) -> Option<usize> {
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