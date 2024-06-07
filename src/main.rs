mod auth;
mod database;
mod room_manager;
mod token_err;
mod user;

use std::sync::{Arc, RwLock};

use database::conn::get_connection_pool;
use dotenvy::dotenv;
use protos::{
    auth::auth_server::AuthServer,
    room::{room_manager_server::RoomManagerServer, RoomList},
};
use tonic::transport::Server;

const MAIN_SERVER_ADDR: &str = "[::1]:50051";

lazy_static::lazy_static! {
    static ref ROOM_LIST: Arc<RwLock<RoomList>> = Arc::new(RwLock::new(RoomList { rooms: Vec::new() }));
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pool = get_connection_pool();

    if pool.test_on_check_out() {
        panic!("Database connection failed")
    }

    let main_server = MAIN_SERVER_ADDR.parse()?;

    Server::builder()
        .add_service(AuthServer::new(auth::Service::new(
            pool.clone().get().unwrap(),
        )))
        .add_service(RoomManagerServer::new(room_manager::Service::new(
            &ROOM_LIST,
            pool.clone().get().unwrap(),
        )))
        .serve(main_server)
        .await?;

    Ok(())
}
