mod auth;
mod data;
mod database;
mod service;
mod utils;

use std::sync::{Arc, Mutex, RwLock};

use auth::auth_service;
use data::{connection_data::ConnectionData, room::Room};
use database::conn::get_connection_pool;
use dotenvy::dotenv;
use protos::{
    auth::auth_server::AuthServer,
    pingpong::{
        ping_pong_service_server::PingPongServiceServer,
        ping_pong_stream_service_server::PingPongStreamServiceServer,
    },
};
use service::{pipo_service, pipo_streaming_service};
use tonic::transport::Server;

const MAIN_SERVER_ADDR: &str = "[::1]:50051";

lazy_static::lazy_static! {
    static ref CONNECTION_DATA_LIST: Arc<RwLock<Vec<ConnectionData>>> = Arc::new(RwLock::new(Vec::new()));
    // static ref ROOM_LIST: Arc<RwLock<Vec<Room>>> = Arc::new(RwLock::new(Vec::new()));
    static ref ROOM_LIST: Arc<Mutex<Vec<Room>>> = Arc::new(Mutex::new(Vec::new()));
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
        .add_service(PingPongServiceServer::new(pipo_service::Service::new()))
        .add_service(PingPongStreamServiceServer::new(
            pipo_streaming_service::Service::new(),
        ))
        .serve(main_server)
        .await?;

    Ok(())
}
