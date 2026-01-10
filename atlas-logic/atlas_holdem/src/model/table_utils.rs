use crate::model::table::{Table, TableError};

impl Table {
    // 从某个座位开始，顺时针查找下一个有玩家的座位
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

    // 判断本轮是否结束
    pub fn betting_round_complete(&mut self)-> bool{
        self.seats.iter().flatten().all(|p| {
            !p.is_active || p.is_all_in || p.has_acted
        })
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

    // 玩家raise支付金额
    pub fn handle_raise_to(&mut self, seat: usize, target: u64) -> Result<(), TableError> {
        let need = {
            let p = self.seats[seat].as_ref().unwrap();
            target.saturating_sub(p.street_bet)
        };
        self.post_amount(seat, need);
        let p = self.seats[seat].as_ref().unwrap();
        // 只有真的超过 current_bet，才算 raise
        if p.street_bet > self.current_bet {
            self.current_bet = p.street_bet;
            self.last_raiser_pos = seat;
        }

        let p = self.seats[seat].as_mut().unwrap();
        p.has_acted = true;
        if p.balance == 0 {
            p.is_all_in = true;
        }
        Ok(())
    }
}