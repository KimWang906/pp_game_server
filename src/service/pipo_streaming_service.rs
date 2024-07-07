use protos::{
    datastreams::{GameEventUpdate, RoomEventUpdate},
    models::{GameSubscription, RoomSubscription},
    pingpong::ping_pong_stream_service_server::PingPongStreamService,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::utils::constants::GameEventCode;

#[derive(Default)]
pub struct Service {}

impl Service {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl PingPongStreamService for Service {
    type GetRoomEventUpdatesStream = ReceiverStream<Result<RoomEventUpdate, Status>>;
    type GetGameEventUpdatesStream = ReceiverStream<Result<GameEventUpdate, Status>>;

    async fn get_room_event_updates(
        &self,
        request: Request<RoomSubscription>,
    ) -> Result<Response<Self::GetRoomEventUpdatesStream>, Status> {
        let data = request.into_inner();

        if data.room_id == GameEventCode::ClearRoomSubscription.as_int() {
            todo!()
        } else {
            todo!()
        }
    }

    async fn get_game_event_updates(
        &self,
        request: Request<GameSubscription>,
    ) -> Result<Response<Self::GetGameEventUpdatesStream>, Status> {
        unimplemented!()
    }
}
