use crate::{
    data::{self, room::Room},
    CONNECTION_DATA_LIST, ROOM_LIST,
};
use std::{convert::TryFrom, time::SystemTime};

#[derive(Debug)]
pub struct CouldNotFindRoomError;

impl TryFrom<u64> for Room {
    type Error = CouldNotFindRoomError;

    fn try_from(room_id: u64) -> Result<Self, Self::Error> {
        let locked_rooms = ROOM_LIST.lock().unwrap();
        Ok(locked_rooms
            .iter()
            .find(|room| room.get_room_id() == room_id)
            .unwrap()
            .clone())
    }
}

pub trait PlayerConversion {
    fn convert_players(players: Vec<protos::models::Player>) -> Vec<data::player::Player>;
}

impl PlayerConversion for Vec<protos::models::Player> {
    fn convert_players(players: Vec<protos::models::Player>) -> Vec<data::player::Player> {
        players
            .iter()
            .map(|player| {
                data::player::Player::new(
                    player.id,
                    player.clone().name,
                    player.ready,
                    player.score,
                )
            })
            .collect()
    }
}

// pub trait  {

// }

pub struct RoomUtils;

impl RoomUtils {
    pub fn all_player_is_ready(room: &Room) -> bool {
        room.get_player_list().iter().all(|player| player.ready)
    }

    pub fn color_is_available(room: &Room, color: protos::models::Color) -> bool {
        room.get_player_list()
            .iter()
            .all(|player| player.color != color as i32)
    }

    pub fn room_name_is_available(name: &str) -> bool {
        ROOM_LIST
            .lock()
            .unwrap()
            .iter()
            .all(|room| room.get_name() != name)
    }

    pub fn generate_room_id(name: String) -> u64 {
        name.chars().fold(0, |acc, c| acc + c as u64)
            * (SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                % 10001)
    }

    pub fn remove_connection(player_id: u32) {
        CONNECTION_DATA_LIST
            .write()
            .unwrap()
            .retain(|conn_data| conn_data.get_player_id() != player_id);
    }
}
