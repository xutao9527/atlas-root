use crate::model::table::Table;

impl Table {


    // 玩家支付金额
    pub fn post_amount(&mut self, seat: usize, amount: u64) {
        let player = self.seats[seat].as_mut().unwrap();
        let actual = amount.min(player.balance);
        player.balance -= actual;
        player.street_bet += actual; // ★★★ 关键
        self.pot += actual;
    }
}