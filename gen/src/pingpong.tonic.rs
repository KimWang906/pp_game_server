// @generated
/// Generated client implementations.
pub mod ping_pong_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct PingPongServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PingPongServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PingPongServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PingPongServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            PingPongServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        ///
        pub async fn broadcast_ball_object(
            &mut self,
            request: impl tonic::IntoRequest<super::super::actions::BroadcastBallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::BroadcastBallResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pingpong.PingPongService/BroadcastBallObject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pingpong.PingPongService", "BroadcastBallObject"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn reconnect(
            &mut self,
            request: impl tonic::IntoRequest<super::super::actions::ReconnectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::ReconnectResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pingpong.PingPongService/Reconnect",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pingpong.PingPongService", "Reconnect"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn host_room(
            &mut self,
            request: impl tonic::IntoRequest<super::super::actions::HostRoomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::HostRoomResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pingpong.PingPongService/HostRoom",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pingpong.PingPongService", "HostRoom"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn get_rooms(
            &mut self,
            request: impl tonic::IntoRequest<super::super::actions::GetRoomsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::GetRoomsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pingpong.PingPongService/GetRooms",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pingpong.PingPongService", "GetRooms"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn add_player(
            &mut self,
            request: impl tonic::IntoRequest<super::super::actions::AddPlayerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::AddPlayerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pingpong.PingPongService/AddPlayer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pingpong.PingPongService", "AddPlayer"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn remove_player(
            &mut self,
            request: impl tonic::IntoRequest<super::super::actions::RemovePlayerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::RemovePlayerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pingpong.PingPongService/RemovePlayer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pingpong.PingPongService", "RemovePlayer"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn set_player_ready(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::actions::SetPlayerReadyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::SetPlayerReadyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pingpong.PingPongService/SetPlayerReady",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pingpong.PingPongService", "SetPlayerReady"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn unsubscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::super::actions::UnsubscribeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::UnsubscribeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pingpong.PingPongService/Unsubscribe",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pingpong.PingPongService", "Unsubscribe"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn broadcast_winner(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::actions::BroadcastWinnerRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::BroadcastWinnerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pingpong.PingPongService/BroadcastWinner",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pingpong.PingPongService", "BroadcastWinner"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn quit_player(
            &mut self,
            request: impl tonic::IntoRequest<super::super::actions::QuitPlayerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::QuitPlayerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pingpong.PingPongService/QuitPlayer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pingpong.PingPongService", "QuitPlayer"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn start_next_round(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::actions::StartNextRoundRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::StartNextRoundResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pingpong.PingPongService/StartNextRound",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pingpong.PingPongService", "StartNextRound"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ping_pong_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PingPongServiceServer.
    #[async_trait]
    pub trait PingPongService: Send + Sync + 'static {
        ///
        async fn broadcast_ball_object(
            &self,
            request: tonic::Request<super::super::actions::BroadcastBallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::BroadcastBallResponse>,
            tonic::Status,
        >;
        ///
        async fn reconnect(
            &self,
            request: tonic::Request<super::super::actions::ReconnectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::ReconnectResponse>,
            tonic::Status,
        >;
        ///
        async fn host_room(
            &self,
            request: tonic::Request<super::super::actions::HostRoomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::HostRoomResponse>,
            tonic::Status,
        >;
        ///
        async fn get_rooms(
            &self,
            request: tonic::Request<super::super::actions::GetRoomsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::GetRoomsResponse>,
            tonic::Status,
        >;
        ///
        async fn add_player(
            &self,
            request: tonic::Request<super::super::actions::AddPlayerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::AddPlayerResponse>,
            tonic::Status,
        >;
        ///
        async fn remove_player(
            &self,
            request: tonic::Request<super::super::actions::RemovePlayerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::RemovePlayerResponse>,
            tonic::Status,
        >;
        ///
        async fn set_player_ready(
            &self,
            request: tonic::Request<super::super::actions::SetPlayerReadyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::SetPlayerReadyResponse>,
            tonic::Status,
        >;
        ///
        async fn unsubscribe(
            &self,
            request: tonic::Request<super::super::actions::UnsubscribeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::UnsubscribeResponse>,
            tonic::Status,
        >;
        ///
        async fn broadcast_winner(
            &self,
            request: tonic::Request<super::super::actions::BroadcastWinnerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::BroadcastWinnerResponse>,
            tonic::Status,
        >;
        ///
        async fn quit_player(
            &self,
            request: tonic::Request<super::super::actions::QuitPlayerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::QuitPlayerResponse>,
            tonic::Status,
        >;
        ///
        async fn start_next_round(
            &self,
            request: tonic::Request<super::super::actions::StartNextRoundRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::actions::StartNextRoundResponse>,
            tonic::Status,
        >;
    }
    ///
    #[derive(Debug)]
    pub struct PingPongServiceServer<T: PingPongService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PingPongService> PingPongServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PingPongServiceServer<T>
    where
        T: PingPongService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/pingpong.PingPongService/BroadcastBallObject" => {
                    #[allow(non_camel_case_types)]
                    struct BroadcastBallObjectSvc<T: PingPongService>(pub Arc<T>);
                    impl<
                        T: PingPongService,
                    > tonic::server::UnaryService<
                        super::super::actions::BroadcastBallRequest,
                    > for BroadcastBallObjectSvc<T> {
                        type Response = super::super::actions::BroadcastBallResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::actions::BroadcastBallRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PingPongService>::broadcast_ball_object(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BroadcastBallObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pingpong.PingPongService/Reconnect" => {
                    #[allow(non_camel_case_types)]
                    struct ReconnectSvc<T: PingPongService>(pub Arc<T>);
                    impl<
                        T: PingPongService,
                    > tonic::server::UnaryService<
                        super::super::actions::ReconnectRequest,
                    > for ReconnectSvc<T> {
                        type Response = super::super::actions::ReconnectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::actions::ReconnectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PingPongService>::reconnect(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReconnectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pingpong.PingPongService/HostRoom" => {
                    #[allow(non_camel_case_types)]
                    struct HostRoomSvc<T: PingPongService>(pub Arc<T>);
                    impl<
                        T: PingPongService,
                    > tonic::server::UnaryService<super::super::actions::HostRoomRequest>
                    for HostRoomSvc<T> {
                        type Response = super::super::actions::HostRoomResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::actions::HostRoomRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PingPongService>::host_room(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HostRoomSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pingpong.PingPongService/GetRooms" => {
                    #[allow(non_camel_case_types)]
                    struct GetRoomsSvc<T: PingPongService>(pub Arc<T>);
                    impl<
                        T: PingPongService,
                    > tonic::server::UnaryService<super::super::actions::GetRoomsRequest>
                    for GetRoomsSvc<T> {
                        type Response = super::super::actions::GetRoomsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::actions::GetRoomsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PingPongService>::get_rooms(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRoomsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pingpong.PingPongService/AddPlayer" => {
                    #[allow(non_camel_case_types)]
                    struct AddPlayerSvc<T: PingPongService>(pub Arc<T>);
                    impl<
                        T: PingPongService,
                    > tonic::server::UnaryService<
                        super::super::actions::AddPlayerRequest,
                    > for AddPlayerSvc<T> {
                        type Response = super::super::actions::AddPlayerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::actions::AddPlayerRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PingPongService>::add_player(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddPlayerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pingpong.PingPongService/RemovePlayer" => {
                    #[allow(non_camel_case_types)]
                    struct RemovePlayerSvc<T: PingPongService>(pub Arc<T>);
                    impl<
                        T: PingPongService,
                    > tonic::server::UnaryService<
                        super::super::actions::RemovePlayerRequest,
                    > for RemovePlayerSvc<T> {
                        type Response = super::super::actions::RemovePlayerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::actions::RemovePlayerRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PingPongService>::remove_player(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemovePlayerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pingpong.PingPongService/SetPlayerReady" => {
                    #[allow(non_camel_case_types)]
                    struct SetPlayerReadySvc<T: PingPongService>(pub Arc<T>);
                    impl<
                        T: PingPongService,
                    > tonic::server::UnaryService<
                        super::super::actions::SetPlayerReadyRequest,
                    > for SetPlayerReadySvc<T> {
                        type Response = super::super::actions::SetPlayerReadyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::actions::SetPlayerReadyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PingPongService>::set_player_ready(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetPlayerReadySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pingpong.PingPongService/Unsubscribe" => {
                    #[allow(non_camel_case_types)]
                    struct UnsubscribeSvc<T: PingPongService>(pub Arc<T>);
                    impl<
                        T: PingPongService,
                    > tonic::server::UnaryService<
                        super::super::actions::UnsubscribeRequest,
                    > for UnsubscribeSvc<T> {
                        type Response = super::super::actions::UnsubscribeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::actions::UnsubscribeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PingPongService>::unsubscribe(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnsubscribeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pingpong.PingPongService/BroadcastWinner" => {
                    #[allow(non_camel_case_types)]
                    struct BroadcastWinnerSvc<T: PingPongService>(pub Arc<T>);
                    impl<
                        T: PingPongService,
                    > tonic::server::UnaryService<
                        super::super::actions::BroadcastWinnerRequest,
                    > for BroadcastWinnerSvc<T> {
                        type Response = super::super::actions::BroadcastWinnerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::actions::BroadcastWinnerRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PingPongService>::broadcast_winner(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BroadcastWinnerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pingpong.PingPongService/QuitPlayer" => {
                    #[allow(non_camel_case_types)]
                    struct QuitPlayerSvc<T: PingPongService>(pub Arc<T>);
                    impl<
                        T: PingPongService,
                    > tonic::server::UnaryService<
                        super::super::actions::QuitPlayerRequest,
                    > for QuitPlayerSvc<T> {
                        type Response = super::super::actions::QuitPlayerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::actions::QuitPlayerRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PingPongService>::quit_player(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QuitPlayerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pingpong.PingPongService/StartNextRound" => {
                    #[allow(non_camel_case_types)]
                    struct StartNextRoundSvc<T: PingPongService>(pub Arc<T>);
                    impl<
                        T: PingPongService,
                    > tonic::server::UnaryService<
                        super::super::actions::StartNextRoundRequest,
                    > for StartNextRoundSvc<T> {
                        type Response = super::super::actions::StartNextRoundResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::actions::StartNextRoundRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PingPongService>::start_next_round(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StartNextRoundSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: PingPongService> Clone for PingPongServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: PingPongService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PingPongService> tonic::server::NamedService for PingPongServiceServer<T> {
        const NAME: &'static str = "pingpong.PingPongService";
    }
}
/// Generated client implementations.
pub mod ping_pong_stream_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct PingPongStreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PingPongStreamServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PingPongStreamServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PingPongStreamServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            PingPongStreamServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        ///
        pub async fn get_room_event_updates(
            &mut self,
            request: impl tonic::IntoRequest<super::super::models::RoomSubscription>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::datastreams::RoomEventUpdate>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pingpong.PingPongStreamService/GetRoomEventUpdates",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "pingpong.PingPongStreamService",
                        "GetRoomEventUpdates",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        ///
        pub async fn get_game_event_updates(
            &mut self,
            request: impl tonic::IntoRequest<super::super::models::GameSubscription>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::datastreams::GameEventUpdate>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pingpong.PingPongStreamService/GetGameEventUpdates",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "pingpong.PingPongStreamService",
                        "GetGameEventUpdates",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ping_pong_stream_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PingPongStreamServiceServer.
    #[async_trait]
    pub trait PingPongStreamService: Send + Sync + 'static {
        /// Server streaming response type for the GetRoomEventUpdates method.
        type GetRoomEventUpdatesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::super::datastreams::RoomEventUpdate,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        ///
        async fn get_room_event_updates(
            &self,
            request: tonic::Request<super::super::models::RoomSubscription>,
        ) -> std::result::Result<
            tonic::Response<Self::GetRoomEventUpdatesStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the GetGameEventUpdates method.
        type GetGameEventUpdatesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::super::datastreams::GameEventUpdate,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        ///
        async fn get_game_event_updates(
            &self,
            request: tonic::Request<super::super::models::GameSubscription>,
        ) -> std::result::Result<
            tonic::Response<Self::GetGameEventUpdatesStream>,
            tonic::Status,
        >;
    }
    ///
    #[derive(Debug)]
    pub struct PingPongStreamServiceServer<T: PingPongStreamService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PingPongStreamService> PingPongStreamServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for PingPongStreamServiceServer<T>
    where
        T: PingPongStreamService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/pingpong.PingPongStreamService/GetRoomEventUpdates" => {
                    #[allow(non_camel_case_types)]
                    struct GetRoomEventUpdatesSvc<T: PingPongStreamService>(pub Arc<T>);
                    impl<
                        T: PingPongStreamService,
                    > tonic::server::ServerStreamingService<
                        super::super::models::RoomSubscription,
                    > for GetRoomEventUpdatesSvc<T> {
                        type Response = super::super::datastreams::RoomEventUpdate;
                        type ResponseStream = T::GetRoomEventUpdatesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::models::RoomSubscription,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PingPongStreamService>::get_room_event_updates(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRoomEventUpdatesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pingpong.PingPongStreamService/GetGameEventUpdates" => {
                    #[allow(non_camel_case_types)]
                    struct GetGameEventUpdatesSvc<T: PingPongStreamService>(pub Arc<T>);
                    impl<
                        T: PingPongStreamService,
                    > tonic::server::ServerStreamingService<
                        super::super::models::GameSubscription,
                    > for GetGameEventUpdatesSvc<T> {
                        type Response = super::super::datastreams::GameEventUpdate;
                        type ResponseStream = T::GetGameEventUpdatesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::models::GameSubscription,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PingPongStreamService>::get_game_event_updates(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetGameEventUpdatesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: PingPongStreamService> Clone for PingPongStreamServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: PingPongStreamService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PingPongStreamService> tonic::server::NamedService
    for PingPongStreamServiceServer<T> {
        const NAME: &'static str = "pingpong.PingPongStreamService";
    }
}
