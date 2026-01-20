use atlas_scheme::proto::holdem::types::PlayerActionKind;

#[derive(Debug)]
pub enum PlayerAct {
    Fold,
    Call,
    Check,
    Bet(u64),
    Raise(u64),
}

impl From<PlayerActionKind> for PlayerAct {
    fn from(action: PlayerActionKind) -> Self {
        match action {
            PlayerActionKind::Fold => PlayerAct::Fold,
            PlayerActionKind::Call => PlayerAct::Call,
            PlayerActionKind::Check => PlayerAct::Check,
            PlayerActionKind::Bet(amount) => PlayerAct::Bet(amount),
            PlayerActionKind::Raise(amount) => PlayerAct::Raise(amount),
        }
    }
}