pub enum GameEventCode {
    NoWinnerIdCode,
    PlayerQuitCode,
    NextRoundCode,
    SkippedTurnCode,
    ClearGameSubscription,
    ClearRoomSubscription,
}

impl GameEventCode {
    pub fn as_int(&self) -> i32 {
        match self {
            GameEventCode::NoWinnerIdCode => -1,
            GameEventCode::PlayerQuitCode => -2,
            GameEventCode::NextRoundCode => -3,
            GameEventCode::SkippedTurnCode => 123,
            GameEventCode::ClearGameSubscription => 456,
            GameEventCode::ClearRoomSubscription => 476,
        }
    }
}
