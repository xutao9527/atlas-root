use crate::model::table::Table;

impl Table {
    // 从某个座位开始，顺时针查找下一个有玩家的座位
    pub fn next_occupied_seat(&self, from: usize) -> Option<usize> {
        let mut i = from;
        loop {
            i = (i + 1) % self.seats.len();
            if i == from {
                return None; // 转了一整圈，没有人能 act
            }
            if let Some(p) = &self.seats[i] {
                if p.is_active && !p.is_all_in {
                    return Some(i);
                }
            }
        }
    }

    // 判断本轮是否结束
    pub fn betting_round_complete(&self) -> bool {
        let mut active_cnt = 0;
        let mut has_pending_actor = false;

        for p in self.seats.iter().flatten() {
            if !p.is_active {
                continue;
            }
            active_cnt += 1;
            if !p.is_all_in && !p.has_acted {
                has_pending_actor = true;
            }
        }
        if active_cnt <= 1 {
            return true;
        }
        !has_pending_actor
    }

    /// 玩家支付金额
    /// 返回 true 表示：此次下注抬高了 table.current_bet（有效 raise）
    pub fn post_amount(&mut self, seat: usize, amount: u64) -> bool {
        let prev_bet = self.current_bet;
        let player = self.seats[seat].as_mut().unwrap();
        let actual = amount.min(player.balance);
        player.balance -= actual;
        player.street_bet += actual;
        self.pot += actual;
        player.has_acted = true;

        // all-in 在这里直接处理
        if player.balance == 0 {
            player.is_all_in = true;
        }
        // ★ 是否形成 raise，只看“结果”
        if player.street_bet > prev_bet {
            self.current_bet = player.street_bet;
            self.last_raiser_pos = seat;
            return true;
        }
        false
    }

    pub fn evaluate_hands(&self) -> Vec<u64>{
        let mut _winners = vec![];


        _winners
    }
}