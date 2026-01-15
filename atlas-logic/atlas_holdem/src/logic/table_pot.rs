use rs_poker::core::Rank;
use crate::model::player::Player;
use crate::model::table::Table;

#[derive(Debug)]
pub struct SidePot {
    /// 这个池子里一共多少钱
    pub amount: u64,
    /// 有资格争夺这个池子的玩家 index
    pub contenders: Vec<usize>,
}

impl Table {
    pub fn build_pots(&self) -> Vec<SidePot> {
        // 取所有下注 > 0 的玩家 (包括 fold 的)
        let mut bet_levels: Vec<u64> = self.seats
            .iter()
            .filter_map(|s| s.as_ref())
            .filter(|p| p.total_bet > 0)
            .map(|p| p.total_bet)
            .collect();

        // 去重 + 排序（从小到大）
        bet_levels.sort_unstable();
        bet_levels.dedup();

        let mut pots = Vec::new();
        let mut prev_level = 0;
        // 每一个下注层级都会形成一个 pot
        for &level in &bet_levels {
            let delta = level - prev_level;
            if delta == 0 {
                continue;
            }

            let mut pot_amount = 0;
            let mut contenders = Vec::new();

            for (idx, seat) in self.seats.iter().enumerate() {
                let Some(player) = seat.as_ref() else { continue };
                if player.total_bet >= level {
                    // 每个满足条件的人，都要为这个 pot 出 delta
                    pot_amount += delta;
                    // 没弃牌的人，才有资格争这个池
                    if player.is_active {
                        contenders.push(idx);
                    }
                }
            }

            pots.push(SidePot {
                amount: pot_amount,
                contenders,
            });

            prev_level = level;
        }
        pots
    }

    pub fn settle_pots(&mut self, pots: Vec<SidePot>) {
        for pot in pots {
            if pot.contenders.is_empty() {
                continue;
            }
            // 找这个池子里的最大牌力
            let max_rank = pot.contenders
                .iter()
                .map(|&idx| {
                    self.seats[idx]
                        .as_ref()
                        .unwrap()
                        .cards_rank
                        .unwrap()
                })
                .max()
                .unwrap();

            // 找所有并列赢家
            let winners: Vec<usize> = pot.contenders
                .into_iter()
                .filter(|&idx| {
                    self.seats[idx]
                        .as_ref()
                        .unwrap()
                        .cards_rank
                        .unwrap() == max_rank
                })
                .collect();

            let share = pot.amount / winners.len() as u64;
            let odd = pot.amount % winners.len() as u64;        // ① odd chip

            for (i, seat_idx) in winners.iter().enumerate() {
                let player = self.seats[*seat_idx].as_mut().unwrap();
                player.balance += share;
                if i == 0 {                                         // ②
                    player.balance += odd;                          // ③ odd chip 给第一个赢家
                }
                player.win = true;
            }
        }
    }
}





fn _mock_player(
    name:&str,
    total_bet: u64,
    rank: Rank,
    is_active: bool,

) -> Player {
    Player {
        id: name.to_string(),
        nickname: name.to_string(),
        balance: 0,
        hand_cards: [None, None],
        cards_str: String::new(),
        sit_out: false,
        win: false,
        cards_rank: Some(rank),
        is_active,
        has_acted: true,
        is_all_in: false,
        street_bet: 0,
        total_bet,
    }
}


#[cfg(test)]
mod tests {
    //use super::*;

    // #[test]
    // fn test_side_pot_example_from_question() {
    //     // A 2000 win
    //     // B 1000 win
    //     // C 1500 win
    //     // D 1000 fold
    //
    //     let mut players = vec![
    //         mock_player("A",2000, Rank::StraightFlush(0), true),  // A
    //         mock_player("B",1000, Rank::StraightFlush(0), true),  // B
    //         mock_player("C",1500, Rank::StraightFlush(0), true),  // C
    //         mock_player("D",1000, Rank::StraightFlush(0), false),  // D fold
    //     ];
    //
    //     let pots = build_pots(&players);
    //     settle_pots(&mut players, pots);
    //
    //     // 总池 = 5500
    //     let total_balance: u64 = players.iter().map(|p| p.balance).sum();
    //     assert_eq!(total_balance, 5500);
    //
    //     // A = 2333
    //     // B = 1333
    //     // C = 1833
    //     // D = 0
    //     assert_eq!(players[0].balance, 2334);
    //     assert_eq!(players[1].balance, 1333);
    //     assert_eq!(players[2].balance, 1833);
    //     assert_eq!(players[3].balance, 0);
    //
    //     assert!(players[0].win);
    //     assert!(players[1].win);
    //     assert!(players[2].win);
    //     assert!(!players[3].win);
    // }
    //
    // #[test]
    // fn test_single_winner_no_side_pot() {
    //     let mut players = vec![
    //         mock_player("A",1000, Rank::StraightFlush(5), true),
    //         mock_player("B",1000, Rank::StraightFlush(2), true),
    //         mock_player("C",1000, Rank::StraightFlush(4), true),
    //     ];
    //
    //     let pots = build_pots(&players);
    //     settle_pots(&mut players, pots);
    //
    //     assert_eq!(players[0].balance, 3000);
    //     assert_eq!(players[1].balance, 0);
    //     assert_eq!(players[2].balance, 0);
    //
    //     assert!(players[0].win);
    //     assert!(!players[1].win);
    //     assert!(!players[2].win);
    // }
    //
    // #[test]
    // fn test_all_in_side_pot() {
    //     // A all-in 500
    //     // B 1000
    //     // C 2000
    //
    //     let mut players = vec![
    //         mock_player("A",500, Rank::StraightFlush(9), true),   // A best
    //         mock_player("A",1000, Rank::StraightFlush(2), true),   // B
    //         mock_player("A",2000, Rank::StraightFlush(1), true),   // C
    //     ];
    //
    //     let pots = build_pots(&players);
    //     settle_pots(&mut players, pots);
    //
    //     // pots:
    //     // main: 1500 (A,B,C) → A
    //     // side1: 1000 (B,C) → B
    //     // side2: 1000 (C) → C
    //
    //     assert_eq!(players[0].balance, 1500);
    //     assert_eq!(players[1].balance, 1000);
    //     assert_eq!(players[2].balance, 1000);
    //
    //     assert!(players[0].win);
    //     assert!(players[1].win);
    //     assert!(players[2].win);
    // }
    //
    // #[test]
    // fn test_everyone_fold_except_one() {
    //     let mut players = vec![
    //         mock_player("A",1000, Rank::StraightFlush(5), false),
    //         mock_player("A",1000, Rank::StraightFlush(5), false),
    //         mock_player("A",1000, Rank::StraightFlush(10), true),
    //     ];
    //
    //     let pots = build_pots(&players);
    //     settle_pots(&mut players, pots);
    //
    //     assert_eq!(players[2].balance, 3000);
    //     assert!(players[2].win);
    // }
}
