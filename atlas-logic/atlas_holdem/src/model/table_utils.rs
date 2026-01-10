use crate::model::table::Table;

impl Table {
    // 判断本轮是否结束
    pub fn betting_round_complete(&mut self)-> bool{
        for p in self.seats.iter().flatten() {
            if !p.is_active {
                continue;
            }
            if p.is_all_in {
                continue;
            }
            if !p.has_acted {
                return false;
            }
            if p.street_bet != self.current_bet {
                return false;
            }
        }
        true
    }

    // 玩家支付金额
    pub fn post_amount(&mut self, seat: usize, amount: u64) {
        let player = self.seats[seat].as_mut().unwrap();
        let actual = amount.min(player.balance);
        player.balance -= actual;
        player.street_bet += actual; // ★★★ 关键
        self.pot += actual;
    }
}