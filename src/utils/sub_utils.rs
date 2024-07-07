use crate::{data::room::Room, CONNECTION_DATA_LIST};

pub enum Type {
    ROOM,
    GAME,
}

pub fn remove_streaming_sub(player_id: u32, subscription_type: Type) {
    let locked_connection_data = CONNECTION_DATA_LIST.read().unwrap();
    let conn_data = locked_connection_data
        .iter()
        .find(|conn_data| conn_data.get_player_id() == player_id)
        .expect("Failed to find player_id");

    match Room::try_from(conn_data.get_room_id()) {
        Ok(mut current_room) => {
            let subscription = current_room
                .get_mut_room_event_subscriber()
                .iter()
                .position(|s| s.get_subscription().player_id == player_id)
                .unwrap();

            match subscription_type {
                Type::ROOM => {
                    dbg!(
                        "RoomStreamingSub: Remove subscription: {:?}, Addr: {}",
                        subscription,
                        conn_data.get_remote_address()
                    );

                    current_room
                        .get_mut_room_event_subscriber()
                        .remove(subscription);
                }
                Type::GAME => {
                    dbg!(
                        "GameStreamingSub: Remove subscription: {:?}, Addr: {}",
                        subscription,
                        conn_data.get_remote_address()
                    );

                    current_room
                        .get_mut_game_event_subscriber()
                        .remove(subscription);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to find room_id: {:?}", e);
        }
    }
}
