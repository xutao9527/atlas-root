use ulid::Ulid;
use crate::model::deck::AtlasDeck;
use crate::model::player::Player;
use crate::model::table::{Table};
use crate::model::table_err::TableError;
use crate::model::table_state::TableState;
use crate::model::table_street::TableStreet;

impl Table {
    ///  新建桌(私有函数)
    fn new(size: usize, small_blind: u64, big_blind: u64) -> Self {
        Self {
            id: Ulid::new().to_string(),
            seats: vec![None; size],
            state: TableState::Waiting,
            street: TableStreet::PreFlop,
            hand_id: String::new(),
            small_blind_amount: small_blind,
            big_blind_amount: big_blind,
            pot: 0,
            current_bet: 0,
            dealer_pos: 0,
            small_blind_pos: 0,
            big_blind_pos: 0,
            current_turn: 0,
            last_raiser_pos: 0,
            deck: AtlasDeck::new(),
            community_cards: Default::default(),
            table_log: Default::default(),
            act_log: Default::default(),
            street_log: Default::default(),
        }
    }

    ///  新建6人桌(公开函数)
    pub fn new_six(small_blind: u64, big_blind: u64) -> Self {
        Self::new(6, small_blind, big_blind)
    }

    ///  新建10人桌(公开函数)
    pub fn new_ten(small_blind: u64, big_blind: u64) -> Self {
        Self::new(10, small_blind, big_blind)
    }

    /// 玩家坐下
    pub fn sit(&mut self,  seat: usize, mut player: Player) -> Result<(), TableError> {
        // ===== 1. seat 索引校验 =====
        if seat >= self.seats.len() {
            return Err(TableError::InvalidSeat);
        }
        // ===== 2. 是否已经在桌上（自己）=====
        if let Some(existing_seat) = self.find_seat_index_by_player_id(&player.id) {
            if existing_seat == seat {
                return Ok(());
            } else {
                // 已经在其他 seat（重连 / 重复请求）
                return Ok(()); // 或 Err(AlreadySeated { seat: existing_seat })
            }
        }
        // ===== 3. seat 是否被“别人”占 =====
        if self.seats[seat].is_some() {
            return Err(TableError::SeatOccupied);
        }
        // ===== 4. buy-in 校验（桌子规则）=====
        let min_buy_in = self.big_blind_amount * 20;
        let max_buy_in = self.big_blind_amount * 100;
        if player.balance < min_buy_in || player.balance > max_buy_in {
            return Err(TableError::InvalidBuyIn {
                min: min_buy_in,
                max: max_buy_in,
                actual: player.balance,
            });
        }
        // ===== 5. 规范化玩家初始状态 =====
        player.hand_cards = [None, None];
        player.sit_out = false;
        player.win = false;
        player.cards_rank = None;
        player.is_active = false;        // 坐下 ≠ 参与当前 hand
        player.has_acted = false;
        player.is_all_in = false;
        player.street_bet = 0;
        player.total_bet = 0;
        // ===== 6. 放入座位 =====

        self.seats[seat] = Some(player);
        Ok(())
    }

    pub fn leave(&mut self, player_id: &str) -> Result<(), TableError> {
        let seat = self
            .find_seat_index_by_player_id(player_id)
            .ok_or(TableError::PlayerNotAtTable)?;

        let player = self.seats[seat]
            .as_mut()
            .ok_or(TableError::PlayerNotAtTable)?;

        if player.is_active {
            // 正在当前局中：标记下局不玩
            player.sit_out = true;
        } else {
            // 不在当前局中：可以直接离桌
            self.seats[seat] = None;
        }
        self.seats[seat] = None;
        Ok(())
    }

    /// 开始新的一局（**系统行为**） 不是玩家指令
    pub fn start(&mut self) -> Result<(), TableError> {
        if self.state != TableState::Waiting && self.state != TableState::Concluding {
            return Err(TableError::InvalidState);
        }
        if self.seats.iter().filter(|s| s.is_some()).count() < 2 {
            return Err(TableError::NotEnoughPlayers);
        }
        self.state = TableState::Preparing;                                             // 进入战前阶段
        // ======================================= init new hand =======================================
        // <-------------------------------------- reset player -------------------------------------->
        // 重置所有玩家的状态
        for seat  in self.seats.iter_mut() {
            match seat {
                Some(p) if p.sit_out => {
                    // 下局不玩，直接离桌
                    *seat = None;
                }
                Some(p) => {
                    // 继续玩的玩家，重置局内状态
                    p.reset();
                }
                None => {}
            }
        }
        // <-------------------------------------- reset table -------------------------------------->
        self.hand_id = Ulid::new().to_string();                                         // 生成hand_id
        self.street = TableStreet::PreFlop;                                             // 更新状态机
        self.pot = 0;                                                                   // 更新底池
        self.current_bet = 0;                                                           // 更新当前下注额
        // 更行并推进庄家按钮
        self.dealer_pos = self.next_occupied_seat(self.dealer_pos).unwrap();
        self.small_blind_pos = self.next_occupied_seat(self.dealer_pos).unwrap();
        self.big_blind_pos   = self.next_occupied_seat(self.small_blind_pos).unwrap();
        // 盲位扣钱
        self.post_amount(self.small_blind_pos, self.small_blind_amount);                // 扣小盲注（强制）
        self.seats[self.small_blind_pos].as_mut().unwrap().has_acted = false;
        self.post_amount(self.big_blind_pos, self.big_blind_amount);                    // 扣大盲注（强制）
        self.seats[self.big_blind_pos].as_mut().unwrap().has_acted = false;

        self.current_bet = self.big_blind_amount;                                       // 更新当前下注额(大盲)
        self.current_turn = self.next_occupied_seat(self.big_blind_pos).unwrap();       // 更新枪口位置(大盲左手)
        self.last_raiser_pos = self.big_blind_pos;                                      // 更新加注者位置(大盲)
        // <-------------------------------------- reset cards -------------------------------------->
        self.deck.shuffle();                                                            // 洗牌
        // 给每个玩家发两张底牌
        for p in self.seats.iter_mut().flatten() {
            p.hand_cards[0] = self.deck.deal_one();
            p.hand_cards[1] = self.deck.deal_one();
        }// 清空公共牌
        self.community_cards = [None; 5];
        // self.community_cards[0] = self.deck.deal_one();
        // self.community_cards[1] = self.deck.deal_one();
        // self.community_cards[2] = self.deck.deal_one();
        // =============================================================================================
        self.street_log.clear();                                                        // 清理下注阶段日志
        self.state = TableState::Battling;                                              // 进入对战阶段
        Ok(())
    }
}
