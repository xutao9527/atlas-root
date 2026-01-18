#[derive(Debug)]
pub enum PlayerAction {
    Fold,
    Call,
    Check,
    Bet(u64),
    Raise(u64),
}
