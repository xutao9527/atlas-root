use tracing::debug;
use crate::model::player_act::{PlayerAct};
use crate::model::table::{Table, };
use crate::model::table_err::TableError;
use crate::model::table_state::TableState;
use crate::model::table_street::TableStreet;

impl Table {
    /// 玩家行动
    pub fn act(&mut self, player_id: String, action: PlayerAct) -> Result<(), TableError> {
        // ======================================= 基础校验 =======================================
        if self.state != TableState::Battling {
            return Err(TableError::InvalidState);
        }
        let seat = self
            .find_seat_index_by_player_id(&player_id)
            .ok_or(TableError::PlayerNotAtTable)?;
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
            PlayerAct::Fold => {
                let p = self.seats[seat].as_mut().unwrap();
                p.is_active = false;
                p.has_acted = true;
                p.acted_view = "fold".to_string();
            }
            PlayerAct::Check => {
                if self.current_bet != 0 {
                    return Err(TableError::InvalidAction);
                }
                let p = self.seats[seat].as_mut().unwrap();
                p.has_acted = true;
                p.acted_view = "check".to_string();
            }
            PlayerAct::Call => {
                if self.current_bet == 0 {
                    return Err(TableError::InvalidAction);
                }
                let need = {
                    let p = self.seats[seat].as_mut().unwrap();
                    p.acted_view = "call".to_string();
                    self.current_bet - p.street_bet
                };
                self.post_amount(seat, need);
            }
            PlayerAct::Bet(amount) => {
                if self.current_bet != 0 || amount == 0 {
                    return Err(TableError::InvalidAction);
                }
                let p = self.seats[seat].as_mut().unwrap();
                p.acted_view = "bet".to_string();
                reopened_betting = self.post_amount(seat, amount);
                //self.write_act_log(&format!("{}",p.acted_view))
            }
            PlayerAct::Raise(amount) => {
                if self.current_bet == 0  {
                    return Err(TableError::InvalidAction);
                }
                let player = self.seats[seat].as_ref().unwrap();
                if amount < self.current_bet - player.street_bet {
                    return Err(TableError::InvalidAction);
                }
                let need = {
                    let p = self.seats[seat].as_mut().unwrap();
                    p.acted_view = "raise".to_string();
                    amount - p.street_bet
                };
                reopened_betting = self.post_amount(seat, need);
            }
        }
        // 记录 act 日志
        let p = self.seats[seat].as_ref().unwrap();
        self.write_act_log(&format!(
            "HAND {} | [{}] act {} ",
            self.hand_id, p.nickname, p.acted_view)
        );
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
            // 更新行动者标记
            self.current_turn = self
                .next_occupied_seat(self.current_turn)
                .ok_or(TableError::InvalidState)?;
        }
        // 设置可用动作标识
        self.current_turn_act.set_available(self.current_bet);
        Ok(())
    }

    /// 阶段结束函数
    fn end_betting_round(&mut self) {
        debug!("betting round finished: {:?}", self.street);
        self.write_street_log();
        // 清空 street 状态
        for p in self.seats.iter_mut().flatten() {
            p.street_bet = 0;
            p.has_acted = false;
            p.acted_view = "".to_string();
        }
        self.advance_street();
    }

    /// 阶段跨越函数
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
