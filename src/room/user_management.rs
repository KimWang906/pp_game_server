use crate::{
    auth_service::{CheckUser, ToStudentId},
    database::{conn::PgPooledConnection, models::DatabaseUser},
};
use protos::room::{
    room_user_manager_server::RoomUserManager, JoinRoomRequest, RoomList, RoomUser, RoomUserList,
};
use std::sync::{Arc, Mutex, RwLock};
use tonic::{Request, Response, Status};

pub struct Service<'a> {
    pub rooms: &'a Arc<RwLock<RoomList>>,
    pub database: Arc<Mutex<PgPooledConnection>>,
}

impl<'a> Service<'a> {
    pub fn new(rooms: &'a Arc<RwLock<RoomList>>, database: PgPooledConnection) -> Self {
        Self {
            database: Arc::new(Mutex::new(database)),
            rooms,
        }
    }
}

#[tonic::async_trait]
impl<'a> RoomUserManager for Service<'a>
where
    'a: 'static,
{
    async fn join_room(
        &self,
        request: Request<JoinRoomRequest>,
    ) -> Result<Response<pbjson_types::Empty>, Status> {
        let mut conn = self.database.lock().unwrap();
        let mut vaildate_token = CheckUser::new(&request, &mut conn);

        // check if the token is valid
        vaildate_token.check();

        let data = request.into_inner();

        let student_id = vaildate_token.to_student_id();
        let user =
            DatabaseUser::find_by_student_id(&mut conn, &student_id).expect("User not found!");

        let mut locked_rooms = self.rooms.write().unwrap();

        let room_id_of_vec = data.id.checked_sub(1).expect("Invalid room id") as usize;
        match locked_rooms.rooms.get_mut(room_id_of_vec) {
            Some(room) => {
                let members = room.members.as_mut().unwrap();

                if members.users.len() >= max_size(room.max_size) {
                    return Err(Status::out_of_range(format!("Room {} is full!", room.name)));
                }

                if !(members.student_id_exists(&user.student_id)) {
                    // Add User
                    members.users.push(RoomUser {
                        username: user.username,
                        student_id: user.student_id,
                    });
                    DatabaseUser::update_room_id(&mut conn, &student_id, Some(room.id))
                        .expect("Error updating room id");
                } else {
                    return Err(Status::already_exists("User already in room!"));
                }
            }
            None => {
                return Err(Status::not_found("Room not found"));
            }
        };

        Ok(Response::new(pbjson_types::Empty {}))
    }

    async fn leave_room(
        &self,
        request: Request<pbjson_types::Empty>,
    ) -> Result<Response<pbjson_types::Empty>, Status> {
        let mut conn = self.database.lock().unwrap();
        let mut vaildate_token = CheckUser::new(&request, &mut conn);

        // check if the token is valid
        vaildate_token.check();

        let student_id = vaildate_token.to_student_id();
        let user =
            DatabaseUser::find_by_student_id(&mut conn, &student_id).expect("User not found!");

        let mut locked_rooms = self.rooms.write().unwrap();

        match user.entered_room_id {
            Some(room_id) => {
                let room_id_of_vec = room_id - 1;
                if room_id_of_vec.is_negative() {
                    return Err(Status::unavailable("Room id is negative"));
                }

                match locked_rooms.rooms.get_mut(room_id_of_vec as usize) {
                    Some(room) => {
                        let members = room.members.as_mut().unwrap();

                        if members.student_id_exists(&user.student_id) {
                            // Remove User
                            members
                                .users
                                .retain(|member| member.student_id != user.student_id);
                            DatabaseUser::update_room_id(&mut conn, &user.student_id, None)
                                .expect("Error updating room id")
                        } else {
                            return Err(Status::not_found("User not in room!"));
                        }
                    }
                    None => return Err(Status::not_found("Room not found")),
                }
            }
            None => {
                return Err(Status::not_found("User not in room!"));
            }
        }

        Ok(Response::new(pbjson_types::Empty {}))
    }
}

fn max_size(s: i32) -> usize {
    match s {
        0 => 2,
        1 => 4,
        _ => 2,
    }
}

trait IsExistStudentId {
    fn student_id_exists(&self, student_id: &str) -> bool;
}

impl IsExistStudentId for RoomUserList {
    fn student_id_exists(&self, student_id: &str) -> bool {
        self.users
            .iter()
            .any(|member| member.student_id == student_id)
    }
}
