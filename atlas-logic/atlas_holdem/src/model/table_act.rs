use ulid::Ulid;
use crate::model::card::Deck;
use crate::model::player::Player;
use crate::model::table::{Table, TableError, TableState, TableStreet};

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
            deck: Deck::new(),
            community_cards: Default::default(),
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

    /// 开始新的一局（**系统行为**） 不是玩家指令
    pub fn start(&mut self) -> Result<(), TableError> {
        if self.state != TableState::Waiting {
            return Err(TableError::InvalidState);
        }
        if self.seats.iter().filter(|s| s.is_some()).count() < 2 {
            return Err(TableError::NotEnoughPlayers);
        }
        self.state = TableState::Preparing;                                             // 进入战前阶段
        // ======================================= init new hand =======================================
        // <-------------------------------------- reset player -------------------------------------->
        // 重置所有玩家的状态
        for p in self.seats.iter_mut().flatten() {
            p.is_active = true;                                                         // 更新玩家状态
            p.has_acted = false;                                                        // 更新行动状态
            p.is_all_in = false;                                                        // 更新all_in状态
            p.street_bet = 0;                                                           // 更新本轮投注额
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

        self.post_amount(self.small_blind_pos, self.small_blind_amount);                // 扣小盲注（强制）
        self.post_amount(self.big_blind_pos, self.big_blind_amount);                    // 扣大盲注（强制）
        self.current_bet = self.big_blind_amount;                                       // 更新当前下注额(大盲)
        self.last_raiser_pos = self.big_blind_pos;                                      // 更新加注者位置(大盲)
        self.current_turn = self.next_occupied_seat(self.big_blind_pos).unwrap();       // 更新枪口位置(大盲左手)
        // <-------------------------------------- reset cards -------------------------------------->
        self.deck.shuffle();                                                            // 洗牌
        // 给每个玩家发两张底牌
        for p in self.seats.iter_mut().flatten() {
            p.hole_cards[0] = self.deck.deal_one();
            p.hole_cards[1] = self.deck.deal_one();
        }// 清空公共牌
        self.community_cards = [None; 5];
        // =============================================================================================
        self.state = TableState::Battling;                                              // 进入对战阶段
        Ok(())
    }
}
