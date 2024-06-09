mod auth;
mod database;
mod room;

use std::sync::{Arc, RwLock};

use auth::auth_service;
use database::conn::get_connection_pool;
use dotenvy::dotenv;
use protos::{
    auth::auth_server::AuthServer,
    room::{
        room_manager_server::RoomManagerServer, room_user_manager_server::RoomUserManagerServer,
        RoomList,
    },
};
use room::{room_manager, user_management};
use tonic::transport::Server;

const MAIN_SERVER_ADDR: &str = "[::1]:50051";

lazy_static::lazy_static! {
    static ref ROOM_LIST: Arc<RwLock<RoomList>> = Arc::new(RwLock::new(RoomList { rooms: Vec::new() }));
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pool = get_connection_pool();

    let main_server = MAIN_SERVER_ADDR.parse()?;

    Server::builder()
        .add_service(AuthServer::new(auth_service::Service::new(
            pool.clone().get().unwrap(),
        )))
        .add_service(RoomManagerServer::new(room_manager::Service::new(
            &ROOM_LIST,
            pool.clone().get().unwrap(),
        )))
        .add_service(RoomUserManagerServer::new(user_management::Service::new(
            &ROOM_LIST,
            pool.clone().get().unwrap(),
        )))
        .serve(main_server)
        .await?;

    Ok(())
}
