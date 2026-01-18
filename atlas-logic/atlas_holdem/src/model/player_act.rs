use atlas_scheme::proto::holdem::types::PlayerActionKind;

#[derive(Debug)]
pub enum PlayerAction {
    Fold,
    Call,
    Check,
    Bet(u64),
    Raise(u64),
}

impl From<PlayerActionKind> for PlayerAction {
    fn from(action: PlayerActionKind) -> Self {
        match action {
            PlayerActionKind::Fold => PlayerAction::Fold,
            PlayerActionKind::Call => PlayerAction::Call,
            PlayerActionKind::Check => PlayerAction::Check,
            PlayerActionKind::Bet(amount) => PlayerAction::Bet(amount),
            PlayerActionKind::Raise(amount) => PlayerAction::Raise(amount),
        }
    }
}