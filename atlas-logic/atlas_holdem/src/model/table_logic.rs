use crate::model::card::Deck;
use crate::model::table::{TableStreet, Table, TableState};
use ulid::Ulid;

impl Table {
    fn new(size: usize, small_blind: u64, big_blind: u64) -> Self {
        Self {
            id: Ulid::new().to_string(),
            seats: vec![None; size],
            state: TableState::Waiting,
            street: TableStreet::PreFlop,
            hand_id: String::new(),
            small_blind_amount: small_blind,
            big_blind_amount: big_blind,
            pot: 0,
            current_bet: 0,
            dealer_pos: 0,
            small_blind_pos: 0,
            big_blind_pos: 0,
            current_turn: 0,
            last_raiser_pos: 0,
            deck: Deck::new(),
            community_cards: Default::default(),
        }
    }

    pub fn new_six(small_blind: u64, big_blind: u64) -> Self {
        Self::new(6, small_blind, big_blind)
    }

    pub fn new_ten(small_blind: u64, big_blind: u64) -> Self {
        Self::new(10, small_blind, big_blind)
    }
}
