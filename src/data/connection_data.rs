pub struct ConnectionData {
    session_id: String,
    remote_address: String,
    player_id: u32,
    room_id: u64,
}

impl ConnectionData {
    pub fn new(session_id: String, remote_address: String, player_id: u32, room_id: u64) -> Self {
        ConnectionData {
            session_id,
            remote_address,
            player_id,
            room_id,
        }
    }

    pub fn get_session_id(&self) -> &String {
        &self.session_id
    }

    pub fn get_remote_address(&self) -> &String {
        &self.remote_address
    }

    pub fn get_player_id(&self) -> u32 {
        self.player_id
    }

    pub fn get_room_id(&self) -> u64 {
        self.room_id
    }

    pub fn set_room_id(&mut self, room_id: u64) {
        self.room_id = room_id;
    }
}

struct FailedToConvertConnectionData {}

trait TryFromSessionId<T> {
    type Error;

    fn try_from(value: T, session_id: String) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl TryFromSessionId<Vec<ConnectionData>> for ConnectionData {
    type Error = FailedToConvertConnectionData;

    fn try_from(value: Vec<ConnectionData>, session_id: String) -> Result<Self, Self::Error> {
        for connection_data in value {
            if connection_data.get_session_id() == &session_id {
                return Ok(connection_data);
            }
        }

        Err(FailedToConvertConnectionData {})
    }
}
