use std::{
    cell::RefCell,
    sync::{Arc, Mutex, RwLock},
};

use protos::room::{
    room_manager_server::RoomManager, CreateRoomRequest, DeleteRoomRequest, Room, RoomInfoRequest,
    RoomList, RoomStatus, RoomUser, RoomUserList,
};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::{
    auth_service::CheckUser,
    database::{conn::PgPooledConnection, models::DatabaseUser},
};

pub struct Service<'a> {
    pub max_id: Arc<Mutex<RefCell<i64>>>,
    pub rooms: &'a Arc<RwLock<RoomList>>,
    pub database: Arc<Mutex<PgPooledConnection>>,
}

impl<'a> Service<'a> {
    pub fn new(rooms: &'a Arc<RwLock<RoomList>>, database: PgPooledConnection) -> Self {
        Self {
            max_id: Arc::new(Mutex::new(RefCell::new(1))),
            database: Arc::new(Mutex::new(database)),
            rooms,
        }
    }
}

#[tonic::async_trait]
impl<'a> RoomManager for Service<'a>
where
    'a: 'static,
{
    type ListRoomsStream = ReceiverStream<Result<Room, Status>>;
    type WatchRoomInfoStream = ReceiverStream<Result<Room, Status>>;

    async fn create_room(
        &self,
        request: Request<CreateRoomRequest>,
    ) -> Result<Response<pbjson_types::Empty>, Status> {
        let mut conn = self.database.lock().unwrap();
        CheckUser::new(&request, &mut conn).check();

        let data = request.into_inner();

        if data.room_name.is_empty() {
            return Err(Status::invalid_argument("Room name cannot be empty"));
        }

        if data.owner_name.is_empty() {
            return Err(Status::invalid_argument("Owner name cannot be empty"));
        }

        let user_data = DatabaseUser::find_by_username(&mut conn, &data.owner_name).unwrap();

        // Create the owner
        let owner = RoomUser {
            username: user_data.username,
            student_id: user_data.student_id.clone(),
        };

        let locked_id = self.max_id.lock().unwrap();
        let mut locked_rooms = self.rooms.write().unwrap();

        // Create the room
        locked_rooms.rooms.push(Room {
            status: Some(RoomStatus::Open as i32),
            id: *locked_id.borrow(),
            name: data.room_name,
            max_size: data.max_size,
            owner: Some(owner.clone()),
            members: Some(RoomUserList { users: vec![owner] }),
        });

        // Update the room ID for the owner
        DatabaseUser::update_room_id(
            &mut conn,
            user_data.student_id.as_str(),
            Some(*locked_id.borrow()),
        )
        .expect("Error while updating the room ID");

        *locked_id.borrow_mut() += 1;

        Ok(Response::new(pbjson_types::Empty {}))
    }

    async fn delete_room(
        &self,
        request: Request<DeleteRoomRequest>,
    ) -> Result<Response<pbjson_types::Empty>, Status> {
        let mut conn = self.database.lock().unwrap();
        let mut vaildate_token = CheckUser::new(&request, &mut conn);

        // check if the token is valid
        vaildate_token.check();

        let data = request.into_inner();

        if data.id == 0 {
            return Err(Status::invalid_argument("Room ID cannot be 0"));
        }

        let room_id_of_vec = data.id - 1;

        let mut locked_rooms = self.rooms.write().unwrap();
        let room = match locked_rooms.rooms.get(room_id_of_vec as usize) {
            Some(room) => {
                // validate the owner of the room
                let owner_student_id = room.owner.as_ref().unwrap().student_id.as_str();
                if !vaildate_token.check_owner(owner_student_id) {
                    return Err(Status::permission_denied(
                        "You are not the owner of the room",
                    ));
                }
                room
            }
            None => {
                return Err(Status::not_found("Room not found"));
            }
        };

        // Remove users from the room
        for user in room.members.as_ref().unwrap().users.iter() {
            DatabaseUser::cleanup_room_userlist(&mut conn, &user.student_id)
        }

        // Remove the room
        locked_rooms.rooms.remove(room_id_of_vec as usize);

        println!("Room {} deleted", data.id);

        Ok(Response::new(pbjson_types::Empty {}))
    }

    async fn list_rooms(
        &self,
        request: Request<pbjson_types::Empty>,
    ) -> Result<Response<Self::ListRoomsStream>, Status> {
        let mut conn = self.database.lock().unwrap();
        CheckUser::new(&request, &mut conn).check();

        let locked_rooms = self
            .rooms
            .read()
            .map_err(|e| Status::internal(format!("Error while reading the room list: {:?}", e)))
            .unwrap()
            .clone();

        let (tx, rx): (Sender<Result<Room, Status>>, Receiver<Result<Room, Status>>) =
            mpsc::channel(128);
        tokio::spawn(async move {
            for room in locked_rooms.rooms.iter() {
                tx.send(Ok(room.clone())).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn watch_room_info(
        &self,
        request: Request<RoomInfoRequest>,
    ) -> Result<Response<Self::WatchRoomInfoStream>, Status> {
        let mut conn = self.database.lock().unwrap();
        CheckUser::new(&request, &mut conn).check();

        let data = request.into_inner();

        let locked_rooms = self
            .rooms
            .read()
            .map_err(|e| Status::internal(format!("Error while reading the room list: {:?}", e)))
            .unwrap()
            .clone();

        let (tx, rx): (Sender<Result<Room, Status>>, Receiver<Result<Room, Status>>) =
            mpsc::channel(128);
        tokio::spawn(async move {
            let room_id_of_vec = data.id - 1;
            match locked_rooms.rooms.get(room_id_of_vec as usize) {
                Some(room) => tx.send(Ok(room.clone())).await.unwrap(),

                None => tx
                    .send(Err(Status::unknown("Could not get room")))
                    .await
                    .unwrap(),
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
