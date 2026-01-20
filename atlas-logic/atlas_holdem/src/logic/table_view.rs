use std::fmt;
use std::process::Command;
use crate::model::table::Table;

fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        // 其他系统用 ANSI 转义码
        print!("\x1B[2J\x1B[1;1H");
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // \x1B[2J: 清屏, \x1B[1;1H: 光标移动到 (1,1)
        //writeln!(f, "\x1B[2J\x1B[1;1H")?;
        clear_screen();
        writeln!(f, "\n{}", "=".repeat(80))?;
        writeln!(f, "TABLE ID: {} | HAND ID: {}", self.id, self.hand_id)?;
        writeln!(f, "STATE: {:?} | STREET: {:?}", self.state, self.street)?;
        writeln!(f, "TABLE ID: {} | STATE: {:?} | street: {:?}", self.id, self.state, self.street)?;
        writeln!(f, "POT: ${} | CURRENT BET:${} | BLIND_AMOUNT :$({}/{})",
                 self.pot, self.current_bet, self.small_blind_amount,self.big_blind_amount)?;
        writeln!(f, "DEALER_POS: {} | SMALL_BLIND_POS BET: {} | BIG_BLIND_POS BET: {} | CURRENT_TURN_POS: {} | LAST_RAISER_POS: {}",
                 self.dealer_pos, self.small_blind_pos, self.big_blind_pos, self.current_turn, self.last_raiser_pos)?;
        writeln!(f, "{}", "-".repeat(80))?;

        // 显示公共牌
        write!(f, "Community Cards: ")?;
        for card in self.community_cards.iter().flatten() {
            write!(f, "{}  ", card)?;  // 用 {} 而不是 {:?}
        }
        writeln!(f)?;
        writeln!(f, "{}", "-".repeat(80))?;

        for (i,r) in self.seats.iter().enumerate() {
            match r {
                Some(p) => {
                    let current_turn_mark = if self.current_turn == i { "*" } else { " " };
                    let last_raiser_mark = if self.last_raiser_pos == i { "R" } else { " " };
                    let is_active_mark = if p.is_active { "√" } else { "×" };
                    let is_all_in_mark = if p.is_all_in { " all-in " } else { "        " };
                    let has_acted_mark =if p.has_acted {"√" } else{ "×" };
                    let dentity_mark = if self.dealer_pos == i {  "D" }
                    else if self.big_blind_pos == i {  "B" }
                    else if self.small_blind_pos == i {  "S"  }
                    else { " " };

                    let hole_cards_str = p.hand_cards
                        .iter()
                        .map(|c| match c {
                            Some(card) => format!("{}", card),
                            None => "??".to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join("  ");

                    // [*][R][S/B/D][i]  nickname balance Cards
                    writeln!(
                        f,
                        " [{turn}][{raiser}][{dealer}]   [{seat}]: {nick} {active} [{all_in:>6}]  [{balance:>6}] [{acted}] [{street_bet:>6}]   hand_cards:{cards}   merge_cards:{merge_cards:>26}  {win}  rank_cards:{rank_cards:?}",
                        turn = current_turn_mark,
                        raiser = last_raiser_mark,
                        dealer = dentity_mark,
                        seat = i,
                        nick = p.nickname,
                        active = is_active_mark,
                        all_in = is_all_in_mark,
                        balance = p.balance,
                        acted = has_acted_mark,
                        street_bet = p.street_bet,
                        cards = hole_cards_str,
                        merge_cards = p.cards_str,
                        win = if p.win { "win" } else { "   " },
                        rank_cards = p.cards_rank
                    )?;
                }
                None => {
                    writeln!(f,"             [{}]  ( Empty )", i)?;
                }
            }
        }

        writeln!(f, "{}", "=".repeat(80))?;
        println!("street log: {:?}", self.street_log);
        writeln!(f, "{}", "command: ")?;
        writeln!(f, "{}", "         [ 1)show; 2)quit; 3)sit <seat> <balance> 4)leave <seat>; 5)start; 6)quick; ]")?;
        writeln!(f, "{}", "         [ 5)act <check> <fold> <call> <bet amount> <raise amount>;]")?;
        Ok(())
    }
}
