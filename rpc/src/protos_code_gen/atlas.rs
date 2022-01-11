/// User related rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateRequest {
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExitRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionResponse {
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
    /// cypher query statetment to execute
    #[prost(string, tag = "2")]
    pub statement: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecResponse {
    /// query result in JSON format
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod atlas_graph_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct AtlasGraphClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AtlasGraphClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AtlasGraphClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AtlasGraphClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            AtlasGraphClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = "/ The following APIs are [STABLE]."]
        pub async fn authenticate(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthenticateRequest>,
        ) -> Result<tonic::Response<super::AuthenticateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/atlas.AtlasGraph/Authenticate");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn ping(
            &mut self,
            request: impl tonic::IntoRequest<super::PingRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/atlas.AtlasGraph/Ping");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn version(
            &mut self,
            request: impl tonic::IntoRequest<super::VersionRequest>,
        ) -> Result<tonic::Response<super::VersionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/atlas.AtlasGraph/Version");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn exec(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecRequest>,
        ) -> Result<tonic::Response<super::ExecResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/atlas.AtlasGraph/Exec");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn exit(
            &mut self,
            request: impl tonic::IntoRequest<super::ExitRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/atlas.AtlasGraph/Exit");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod atlas_graph_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with AtlasGraphServer."]
    #[async_trait]
    pub trait AtlasGraph: Send + Sync + 'static {
        #[doc = "/ The following APIs are [STABLE]."]
        async fn authenticate(
            &self,
            request: tonic::Request<super::AuthenticateRequest>,
        ) -> Result<tonic::Response<super::AuthenticateResponse>, tonic::Status>;
        async fn ping(
            &self,
            request: tonic::Request<super::PingRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn version(
            &self,
            request: tonic::Request<super::VersionRequest>,
        ) -> Result<tonic::Response<super::VersionResponse>, tonic::Status>;
        async fn exec(
            &self,
            request: tonic::Request<super::ExecRequest>,
        ) -> Result<tonic::Response<super::ExecResponse>, tonic::Status>;
        async fn exit(
            &self,
            request: tonic::Request<super::ExitRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct AtlasGraphServer<T: AtlasGraph> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AtlasGraph> AtlasGraphServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AtlasGraphServer<T>
    where
        T: AtlasGraph,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/atlas.AtlasGraph/Authenticate" => {
                    #[allow(non_camel_case_types)]
                    struct AuthenticateSvc<T: AtlasGraph>(pub Arc<T>);
                    impl<T: AtlasGraph> tonic::server::UnaryService<super::AuthenticateRequest> for AuthenticateSvc<T> {
                        type Response = super::AuthenticateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthenticateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).authenticate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AuthenticateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/atlas.AtlasGraph/Ping" => {
                    #[allow(non_camel_case_types)]
                    struct PingSvc<T: AtlasGraph>(pub Arc<T>);
                    impl<T: AtlasGraph> tonic::server::UnaryService<super::PingRequest> for PingSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).ping(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/atlas.AtlasGraph/Version" => {
                    #[allow(non_camel_case_types)]
                    struct VersionSvc<T: AtlasGraph>(pub Arc<T>);
                    impl<T: AtlasGraph> tonic::server::UnaryService<super::VersionRequest> for VersionSvc<T> {
                        type Response = super::VersionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = VersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/atlas.AtlasGraph/Exec" => {
                    #[allow(non_camel_case_types)]
                    struct ExecSvc<T: AtlasGraph>(pub Arc<T>);
                    impl<T: AtlasGraph> tonic::server::UnaryService<super::ExecRequest> for ExecSvc<T> {
                        type Response = super::ExecResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).exec(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/atlas.AtlasGraph/Exit" => {
                    #[allow(non_camel_case_types)]
                    struct ExitSvc<T: AtlasGraph>(pub Arc<T>);
                    impl<T: AtlasGraph> tonic::server::UnaryService<super::ExitRequest> for ExitSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExitRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).exit(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: AtlasGraph> Clone for AtlasGraphServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: AtlasGraph> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AtlasGraph> tonic::transport::NamedService for AtlasGraphServer<T> {
        const NAME: &'static str = "atlas.AtlasGraph";
    }
}
