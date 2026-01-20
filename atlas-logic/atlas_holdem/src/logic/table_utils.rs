use crate::model::table::Table;
use rs_poker::core::Rankable;

impl Table {

    // 根据玩家ID查找座位序号
    pub fn find_seat_index_by_player_id(&self, player_id: &str) -> Option<usize> {
        self.seats.iter().position(|s| {
            s.as_ref().map_or(false, |p| p.id == player_id)
        })
    }

    // 根据座位序号查找玩家ID
    pub fn find_player_id_by_seat_index(&self, seat_index: usize) -> Option<String> {
        self.seats
            .get(seat_index)?
            .as_ref()
            .map(|p| p.id.clone())
    }

    /// 指定座位索引,顺时针查找下一个有玩家的座位
    pub fn next_occupied_seat(&self, seat_index: usize) -> Option<usize> {
        let mut i = seat_index;
        loop {
            i = (i + 1) % self.seats.len();
            if i == seat_index {
                return None; // 转了一整圈，没有人能 act
            }
            if let Some(p) = &self.seats[i] {
                if p.is_active && !p.is_all_in {
                    return Some(i);
                }
            }
        }
    }

    /// 判断本轮是否结束
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
        player.total_bet += actual;
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

    /// 计算玩家牌力值
    pub fn evaluate_hands(&mut self) -> Vec<u64>{
        let mut _winners = vec![];
        self.seats.iter_mut().flatten().for_each(|p| {
            p.cards_str = p.hand_cards
                .iter()
                .flatten()
                .map(|c| format!("{}", c))
                .chain(std::iter::once("|".to_string()))
                .chain(
                    self.community_cards
                        .iter()
                        .flatten()
                        .map(|c| format!("{}", c))
                )
                .collect::<Vec<_>>()
                .join(" ");

            let merge_cards = p
                .hand_cards
                .iter()
                .chain(self.community_cards.iter())
                .flatten()
                .map(|c| {
                    c.into()
                })
                .collect::<Vec<_>>();

            p.cards_rank = Some(merge_cards.rank());
        });

        // 找最大牌力
        let max_rank = self.seats
            .iter()
            .flatten()
            .map(|p| p.cards_rank.unwrap())
            .max()
            .unwrap();

        // 标记赢家（支持平分池）
        for p in self.seats.iter_mut().flatten() {
            p.win = false;
            if p.cards_rank.unwrap() == max_rank {
                p.win = true;
            }
        }
        _winners
    }
}