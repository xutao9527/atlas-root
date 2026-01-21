use atlas_scheme::proto::holdem::types::{PlayerActionKind, PlayerAvailableActView};

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

#[derive(Debug, Clone, Default)]
pub struct PlayerAvailableAct {
    pub fold: bool,
    pub call: bool,
    pub check: bool,
    pub bet: bool,
    pub raise: bool,
}

impl PlayerAvailableAct {
    pub fn set_available(&mut self, current_bet: u64) {
        if current_bet == 0 {
            self.fold = true;
            self.call = false;
            self.check = true;
            self.bet = true;
            self.raise = false;
        } else {
            self.fold = true;
            self.call = true;
            self.check = false;
            self.bet = false;
            self.raise = true;
        }
    }
}

impl From<&PlayerAvailableAct> for PlayerAvailableActView {
    fn from(act: &PlayerAvailableAct) -> Self {
        PlayerAvailableActView {
            fold: act.fold,
            call: act.call,
            check: act.check,
            bet: act.bet,
            raise: act.raise,
        }
    }
}

impl From<PlayerAvailableAct> for PlayerAvailableActView {
    fn from(act: PlayerAvailableAct) -> Self {
        (&act).into()
    }
}