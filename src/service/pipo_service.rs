use protos::{actions::*, pingpong::ping_pong_service_server::PingPongService};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct Service {}

impl Service {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl PingPongService for Service {
    async fn broadcast_ball_object(
        &self,
        request: Request<BroadcastBallRequest>,
    ) -> Result<Response<BroadcastBallResponse>, Status> {
        unimplemented!()
    }

    async fn reconnect(
        &self,
        request: Request<ReconnectRequest>,
    ) -> Result<Response<ReconnectResponse>, Status> {
        unimplemented!()
    }

    async fn host_room(
        &self,
        request: Request<HostRoomRequest>,
    ) -> Result<Response<HostRoomResponse>, Status> {
        unimplemented!()
    }

    async fn get_rooms(
        &self,
        request: Request<GetRoomsRequest>,
    ) -> Result<Response<GetRoomsResponse>, Status> {
        unimplemented!()
    }

    async fn add_player(
        &self,
        request: Request<AddPlayerRequest>,
    ) -> Result<Response<AddPlayerResponse>, Status> {
        unimplemented!()
    }

    async fn remove_player(
        &self,
        request: Request<RemovePlayerRequest>,
    ) -> Result<Response<RemovePlayerResponse>, Status> {
        unimplemented!()
    }

    async fn set_player_ready(
        &self,
        request: Request<SetPlayerReadyRequest>,
    ) -> Result<Response<SetPlayerReadyResponse>, Status> {
        unimplemented!()
    }

    async fn unsubscribe(
        &self,
        request: Request<UnsubscribeRequest>,
    ) -> Result<Response<UnsubscribeResponse>, Status> {
        unimplemented!()
    }

    async fn broadcast_winner(
        &self,
        request: Request<BroadcastWinnerRequest>,
    ) -> Result<Response<BroadcastWinnerResponse>, Status> {
        unimplemented!()
    }

    async fn quit_player(
        &self,
        request: Request<QuitPlayerRequest>,
    ) -> Result<Response<QuitPlayerResponse>, Status> {
        unimplemented!()
    }

    async fn start_next_round(
        &self,
        request: Request<StartNextRoundRequest>,
    ) -> Result<Response<StartNextRoundResponse>, Status> {
        unimplemented!()
    }
}
