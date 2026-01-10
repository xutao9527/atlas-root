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
        self.state = TableState::Preparing;
        // =============== 初始化新的一局 ===============
        // 生成新一局 hand_id
        self.hand_id = Ulid::new().to_string();
        // 初始化 Street
        self.street = TableStreet::PreFlop;
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
        self.post_amount(self.small_blind_pos, self.small_blind_amount);
        self.post_amount(self.big_blind_pos, self.big_blind_amount);

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
}
