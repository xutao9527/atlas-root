pub struct WsSession{
    pub uid: Option<String>,
    pub token: Option<String>,
    pub expire_at: Option<u64>, // unix seconds
}

impl WsSession {
    pub fn new() -> Self {
        Self {
            uid: None,
            token: None,
            expire_at: None,
        }
    }

    pub fn is_authed(&self) -> bool {
        self.uid.is_some()
    }
}