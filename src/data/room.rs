use protos::models::{MaxSize, Player};

use super::{game_event_sub::GameEventSubscription, room_event_sub::RoomEventSubscription};

#[derive(Debug, Clone)]
pub enum Status {
    WAITING,
    PLAING,
}

#[derive(Debug)]
pub struct FailedToGetPlayerIdxError;
pub type GotIdxError = FailedToGetPlayerIdxError;

#[derive(Clone)]
pub struct Room {
    room_id: u64,
    name: String,
    password: String,
    status: Status,
    count: u32,
    player_idx: u32,
    max_player: MaxSize,
    player_list: Vec<Player>,
    room_event_subscriber: Vec<RoomEventSubscription>,
    game_event_subscriber: Vec<GameEventSubscription>,
}

impl Room {
    pub fn new(
        room_id: u64,
        player_list: Vec<Player>,
        count: u32,
        player_idx: u32,
        max_player: MaxSize,
        status: Status,
        name: String,
        password: String,
    ) -> Self {
        Self {
            room_id,
            name,
            password,
            status,
            count,
            player_idx,
            max_player,
            player_list,
            room_event_subscriber: Vec::new(),
            game_event_subscriber: Vec::new(),
        }
    }

    pub fn vaild_password(&self, password: &str) -> bool {
        self.password.eq("-1") || self.password.len() >= 3
    }

    pub fn move_self(self) -> Self {
        self
    }

    /* =========================== Getters and Setters =========================== */

    pub fn get_room_id(&self) -> u64 {
        self.room_id
    }

    pub fn set_room_id(&mut self, room_id: u64) {
        self.room_id = room_id;
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_password(&self) -> &String {
        &self.password
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }

    pub fn set_count(&mut self, count: u32) {
        self.count = count;
    }

    pub fn get_max_player(&self) -> &MaxSize {
        &self.max_player
    }

    pub fn get_player_list(&self) -> &Vec<Player> {
        &self.player_list
    }

    pub fn set_player_list(&mut self, player_list: Vec<Player>) {
        self.player_list = player_list;
    }

    pub fn get_current_player_idx(&self) -> Result<u32, GotIdxError> {
        if self.player_list.is_empty() && self.player_list.get(self.player_idx as usize).is_some() {
            return Err(FailedToGetPlayerIdxError);
        }
        Ok(self.player_idx)
    }

    pub fn set_current_player_idx(&mut self, player_idx: u32) {
        self.player_idx = player_idx;
    }

    pub fn get_room_event_subscriber(&self) -> &Vec<RoomEventSubscription> {
        &self.room_event_subscriber
    }

    pub fn get_mut_room_event_subscriber(&mut self) -> &mut Vec<RoomEventSubscription> {
        &mut self.room_event_subscriber
    }

    pub fn set_room_event_subscriber(&mut self, room_event_subscriber: Vec<RoomEventSubscription>) {
        self.room_event_subscriber = room_event_subscriber;
    }

    pub fn get_game_event_subscriber(&self) -> &Vec<GameEventSubscription> {
        &self.game_event_subscriber
    }

    pub fn get_mut_game_event_subscriber(&mut self) -> &mut Vec<GameEventSubscription> {
        &mut self.game_event_subscriber
    }

    pub fn set_game_event_subscriber(&mut self, game_event_subscriber: Vec<GameEventSubscription>) {
        self.game_event_subscriber = game_event_subscriber;
    }

    pub fn set_player_idx(&mut self, player_idx: u32) {
        self.player_idx = player_idx;
    }
}

trait TryFromRoomId<T> {
    type Error;
    fn try_from(rooms: Vec<Room>, room_id: T) -> Result<T, Self::Error>;
}

impl TryFromRoomId<u64> for String {
    type Error = FailedToGetPlayerIdxError;
    fn try_from(rooms: Vec<Room>, room_id: u64) -> Result<u64, Self::Error> {
        if room_id == 0 {
            return Err(FailedToGetPlayerIdxError);
        }

        Ok(rooms
            .iter()
            .find(|room| room.room_id == room_id)
            .ok_or_else(|| FailedToGetPlayerIdxError)?
            .room_id
            .try_into()
            .unwrap())
    }
}
