use crate::model::player::PlayerAction;
use crate::model::table::{Table, TableState, TableStreet};
use crate::model::table_err::TableError;

impl Table {
    pub fn act(&mut self, seat: usize, action: PlayerAction) -> Result<(), TableError> {
        // ======================================= 基础校验 =======================================
        if self.state != TableState::Battling {
            return Err(TableError::InvalidState);
        }
        if seat != self.current_turn {
            return Err(TableError::InvalidSeat);
        }
        let player_active = {
            let p = self.seats[seat].as_ref().ok_or(TableError::InvalidSeat)?;
            p.is_active && !p.is_all_in
        };
        if !player_active {
            return Err(TableError::InvalidAction);
        }
        // ======================================= 动作处理 =======================================
        // 标记是否发生了 bet / raise
        let mut reopened_betting = false;
        match action {
            PlayerAction::Fold => {
                let p = self.seats[seat].as_mut().unwrap();
                p.is_active = false;
                p.has_acted = true;
            }
            PlayerAction::Check => {
                if self.current_bet != 0 {
                    return Err(TableError::InvalidAction);
                }
                self.seats[seat].as_mut().unwrap().has_acted = true;
            }
            PlayerAction::Call => {
                if self.current_bet == 0 {
                    return Err(TableError::InvalidAction);
                }
                let need = {
                    let p = self.seats[seat].as_ref().unwrap();
                    self.current_bet - p.street_bet
                };
                self.post_amount(seat, need);
            }
            PlayerAction::Bet(amount) => {
                if self.current_bet != 0 || amount == 0 {
                    return Err(TableError::InvalidAction);
                }
                reopened_betting = self.post_amount(seat, amount);
            }
            PlayerAction::Raise(amount) => {
                if self.current_bet == 0 || amount <= self.current_bet {
                    return Err(TableError::InvalidAction);
                }
                let need = {
                    let p = self.seats[seat].as_ref().unwrap();
                    amount - p.street_bet
                };
                reopened_betting = self.post_amount(seat, need);
            }
        }
        // bet / raise 后，其他玩家需要重新行动
        if reopened_betting {
            for (i, p) in self.seats.iter_mut().enumerate() {
                if i == seat {
                    continue;
                }
                if i != seat {
                    if let Some(p) = p {
                        p.has_acted = false;
                    }
                }
            }
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
        self.street_log.push(self.street);
        // 清空 street 状态
        for p in self.seats.iter_mut().flatten() {
            p.street_bet = 0;
            p.has_acted = false;
        }
        self.advance_street();
    }

    fn advance_street(&mut self){
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
                let pots = self.build_pots();
                self.settle_pots(pots);
                return;
            }
        }
        self.evaluate_hands();
        // ======================= 新一轮下注初始化 =======================
        self.last_raiser_pos = self.dealer_pos;
        self.current_bet = 0;

        if self.betting_round_complete() {
            self.end_betting_round();
        }

        // 新一轮下注从庄家左手开始
        self.current_turn = self
            .next_occupied_seat(self.dealer_pos)
            .unwrap_or(self.dealer_pos);
    }
}
