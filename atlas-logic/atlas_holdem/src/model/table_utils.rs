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

    // 玩家支付金额
    pub fn post_amount(&mut self, seat: usize, amount: u64) {
        let player = self.seats[seat].as_mut().unwrap();
        let actual = amount.min(player.balance);
        player.balance -= actual;
        player.street_bet += actual; // ★★★ 关键
        self.pot += actual;
    }

    pub(crate) fn handle_raise_to(&mut self, seat: usize, target: u64) -> Result<(), TableError> {
        let need = {
            let p = self.seats[seat].as_ref().unwrap();
            target.saturating_sub(p.street_bet)
        };
        self.post_amount(seat, need);
        self.current_bet = target;
        self.last_raiser_pos = seat;
        let p = self.seats[seat].as_mut().unwrap();
        p.has_acted = true;
        if p.balance == 0 {
            p.is_all_in = true;
        }
        Ok(())
    }
}