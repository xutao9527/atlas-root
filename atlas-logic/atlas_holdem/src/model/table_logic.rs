use crate::model::player::PlayerAction;
use crate::model::table::{Table, TableError, TableState, TableStreet};

impl Table {
    pub fn act(&mut self, seat: usize, action: PlayerAction) -> Result<(), TableError> {
        // ======================================= 基础校验 =======================================
        if self.state != TableState::Battling {
            return Err(TableError::InvalidState);
        }
        if seat != self.current_turn {
            return Err(TableError::InvalidSeat);
        }
        if self.seats[seat].is_none() {
            return Err(TableError::InvalidSeat);
        }
        if {
            let p = self.seats[seat].as_ref().unwrap();
            !p.is_active || p.is_all_in
        } {
            return Err(TableError::InvalidAction);
        }
        // 标记是否发生了 bet / raise
        let mut reopened_betting = false;
        // ======================================= 动作处理 =======================================
        match action {
            PlayerAction::Fold => {
                let player = self.seats[seat].as_mut().unwrap();
                player.is_active = false;
                player.has_acted = true;
            }
            PlayerAction::Check => {
                if self.current_bet != 0 {
                    return Err(TableError::InvalidAction);
                }
                let player = self.seats[seat].as_mut().unwrap();
                player.has_acted = true;
            }
            PlayerAction::Call => {
                if self.current_bet == 0 {
                    return Err(TableError::InvalidAction);
                }
                let need = {
                    let player = self.seats[seat].as_ref().unwrap();
                    self.current_bet - player.street_bet
                };
                self.post_amount(seat, need);
                let player = self.seats[seat].as_mut().unwrap();
                if player.balance == 0 {
                    player.is_all_in = true;
                }
                player.has_acted = true;
            }

            PlayerAction::Bet(amount) => {
                if self.current_bet != 0 || amount == 0 {
                    return Err(TableError::InvalidAction);
                }
                self.post_amount(seat, amount);
                self.current_bet = amount;
                self.last_raiser_pos = seat;
                reopened_betting = true;
                let player = self.seats[seat].as_mut().unwrap();
                player.has_acted = true;
                if player.balance == 0 {
                    player.is_all_in = true;
                }
            }

            PlayerAction::Raise(amount) => {
                if self.current_bet == 0 || amount <= self.current_bet {
                    return Err(TableError::InvalidAction);
                }
                let need = {
                    let player = self.seats[seat].as_ref().unwrap();
                    amount - player.street_bet
                };
                self.post_amount(seat, need);
                self.current_bet = amount;
                self.last_raiser_pos = seat;
                reopened_betting = true;
                let player = self.seats[seat].as_mut().unwrap();
                player.has_acted = true;
                if player.balance == 0 {
                    player.is_all_in = true;
                }
            }
        }

        // bet / raise 后，其他玩家需要重新行动
        if reopened_betting {
            for (i, p) in self.seats.iter_mut().enumerate() {
                if i != seat {
                    if let Some(p) = p {
                        p.has_acted = false;
                    }
                }
            }
            let player = self.seats[seat].as_mut().unwrap();
            player.has_acted = true;
        }

        // ======================================= 推进下注轮 =======================================
        if self.betting_round_complete() {
            self.end_betting_round();
        } else {
            self.current_turn = self
                .next_occupied_seat(self.current_turn)
                .ok_or(TableError::InvalidState)?;
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

    fn advance_street(&mut self){
        let active_cnt = self
            .seats
            .iter()
            .flatten()
            .filter(|p| p.is_active)
            .count();

        // 只剩一个玩家，直接结束
        if active_cnt <= 1 {
            self.state = TableState::Concluding;
            return;
        }
        match self.street {
            TableStreet::PreFlop => {
                // 发 Flop（三张）
                self.community_cards[0] = self.deck.deal_one();
                self.community_cards[1] = self.deck.deal_one();
                self.community_cards[2] = self.deck.deal_one();
                self.street = TableStreet::Flop;
            }
            TableStreet::Flop => {
                // 发 Turn（一张）
                self.community_cards[3] = self.deck.deal_one();
                self.street = TableStreet::Turn;
            }
            TableStreet::Turn => {
                // 发 River（一张）
                self.community_cards[4] = self.deck.deal_one();
                self.street = TableStreet::River;
            }
            TableStreet::River => {
                // River 后进入结算
                self.state = TableState::Concluding;
                return;
            }
        }
        // ======================= 新一轮下注初始化 =======================
        self.current_bet = 0;
        self.last_raiser_pos = self.dealer_pos;

        // 新一轮下注从庄家左手开始
        self.current_turn = match self.next_occupied_seat(self.dealer_pos) {
            Some(pos) => pos,
            None => {
                self.state = TableState::Concluding;
                return;
            }
        };

        // 如果所有 active 玩家都 all-in，直接快进到结算
        let need_bet = self
            .seats
            .iter()
            .flatten()
            .any(|p| p.is_active && !p.is_all_in);

        if !need_bet {
            // 递归推进，直接发完公共牌
            self.advance_street();
        }

    }
}
