use crate::model::player::Player;

impl Player {
    pub fn reset(&mut self) {
        self.hand_cards = [None; 2];                                                    // 更新手牌
        self.cards_str = "".to_string();                                                // 更新手牌显示
        self.win = false;                                                               // 更新输赢状态
        self.cards_rank = None;                                                         // 更新牌力
        self.is_active = true;                                                          // 更新玩家状态
        self.has_acted = false;                                                         // 更新行动状态
        self.is_all_in = false;                                                         // 更新all_in状态
        self.street_bet = 0;                                                            // 更新本轮投注额
        self.total_bet = 0;                                                             // 更新总投入筹码
    }
}
