// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Color {
    #[prost(float, tag = "1")]
    pub r: f32,
    #[prost(float, tag = "2")]
    pub g: f32,
    #[prost(float, tag = "3")]
    pub b: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vec2 {
    #[prost(float, tag = "1")]
    pub x: f32,
    #[prost(float, tag = "2")]
    pub y: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddItemResponse {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AxesDescriptor {
    #[prost(message, optional, tag = "1")]
    pub origin: ::core::option::Option<Vec2>,
    #[prost(message, optional, tag = "2")]
    pub x_axis: ::core::option::Option<Vec2>,
    #[prost(message, optional, tag = "3")]
    pub y_axis: ::core::option::Option<Vec2>,
    #[prost(float, tag = "4")]
    pub neg_len: f32,
    #[prost(float, tag = "5")]
    pub pos_len: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAxesRequest {
    #[prost(string, tag = "1")]
    pub client_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub axes_descriptor: ::core::option::Option<AxesDescriptor>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SquareDescriptor {
    #[prost(message, optional, tag = "1")]
    pub origin: ::core::option::Option<Vec2>,
    #[prost(message, optional, tag = "2")]
    pub x_axis: ::core::option::Option<Vec2>,
    #[prost(message, optional, tag = "3")]
    pub y_axis: ::core::option::Option<Vec2>,
    #[prost(float, tag = "4")]
    pub lenx: f32,
    #[prost(float, tag = "5")]
    pub leny: f32,
    #[prost(message, optional, tag = "6")]
    pub line_color: ::core::option::Option<Color>,
    #[prost(message, optional, tag = "7")]
    pub tri_color: ::core::option::Option<Color>,
    #[prost(enumeration = "CellType", tag = "8")]
    pub cell_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddSquareRequest {
    #[prost(string, tag = "1")]
    pub client_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub square_descriptor: ::core::option::Option<SquareDescriptor>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CirlceDescriptor {
    #[prost(message, optional, tag = "1")]
    pub center: ::core::option::Option<Vec2>,
    #[prost(float, tag = "2")]
    pub radius: f32,
    #[prost(message, optional, tag = "3")]
    pub line_color: ::core::option::Option<Color>,
    #[prost(message, optional, tag = "4")]
    pub tri_color: ::core::option::Option<Color>,
    #[prost(enumeration = "CellType", tag = "5")]
    pub cell_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCircleRequest {
    #[prost(string, tag = "1")]
    pub client_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub circle_descriptor: ::core::option::Option<CirlceDescriptor>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KillServerRequest {
    #[prost(string, tag = "1")]
    pub client_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KillServerResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CellType {
    None = 0,
    Line = 1,
    Triangle = 2,
}
impl CellType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CellType::None => "NONE",
            CellType::Line => "LINE",
            CellType::Triangle => "TRIANGLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "LINE" => Some(Self::Line),
            "TRIANGLE" => Some(Self::Triangle),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod state_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct StateServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StateServiceClient<tonic::transport::Channel> {
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
    impl<T> StateServiceClient<T>
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
        ) -> StateServiceClient<InterceptedService<T, F>>
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
            StateServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn add_axes(
            &mut self,
            request: impl tonic::IntoRequest<super::AddAxesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddItemResponse>,
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
                "/d2rpc.StateService/AddAxes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("d2rpc.StateService", "AddAxes"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_square(
            &mut self,
            request: impl tonic::IntoRequest<super::AddSquareRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddItemResponse>,
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
                "/d2rpc.StateService/AddSquare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("d2rpc.StateService", "AddSquare"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_circle(
            &mut self,
            request: impl tonic::IntoRequest<super::AddCircleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddItemResponse>,
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
                "/d2rpc.StateService/AddCircle",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("d2rpc.StateService", "AddCircle"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn kill_server(
            &mut self,
            request: impl tonic::IntoRequest<super::KillServerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::KillServerResponse>,
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
                "/d2rpc.StateService/KillServer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("d2rpc.StateService", "KillServer"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod state_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with StateServiceServer.
    #[async_trait]
    pub trait StateService: Send + Sync + 'static {
        async fn add_axes(
            &self,
            request: tonic::Request<super::AddAxesRequest>,
        ) -> std::result::Result<tonic::Response<super::AddItemResponse>, tonic::Status>;
        async fn add_square(
            &self,
            request: tonic::Request<super::AddSquareRequest>,
        ) -> std::result::Result<tonic::Response<super::AddItemResponse>, tonic::Status>;
        async fn add_circle(
            &self,
            request: tonic::Request<super::AddCircleRequest>,
        ) -> std::result::Result<tonic::Response<super::AddItemResponse>, tonic::Status>;
        async fn kill_server(
            &self,
            request: tonic::Request<super::KillServerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::KillServerResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct StateServiceServer<T: StateService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: StateService> StateServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StateServiceServer<T>
    where
        T: StateService,
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
                "/d2rpc.StateService/AddAxes" => {
                    #[allow(non_camel_case_types)]
                    struct AddAxesSvc<T: StateService>(pub Arc<T>);
                    impl<
                        T: StateService,
                    > tonic::server::UnaryService<super::AddAxesRequest>
                    for AddAxesSvc<T> {
                        type Response = super::AddItemResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddAxesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StateService>::add_axes(&inner, request).await
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
                        let method = AddAxesSvc(inner);
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
                "/d2rpc.StateService/AddSquare" => {
                    #[allow(non_camel_case_types)]
                    struct AddSquareSvc<T: StateService>(pub Arc<T>);
                    impl<
                        T: StateService,
                    > tonic::server::UnaryService<super::AddSquareRequest>
                    for AddSquareSvc<T> {
                        type Response = super::AddItemResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddSquareRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StateService>::add_square(&inner, request).await
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
                        let method = AddSquareSvc(inner);
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
                "/d2rpc.StateService/AddCircle" => {
                    #[allow(non_camel_case_types)]
                    struct AddCircleSvc<T: StateService>(pub Arc<T>);
                    impl<
                        T: StateService,
                    > tonic::server::UnaryService<super::AddCircleRequest>
                    for AddCircleSvc<T> {
                        type Response = super::AddItemResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddCircleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StateService>::add_circle(&inner, request).await
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
                        let method = AddCircleSvc(inner);
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
                "/d2rpc.StateService/KillServer" => {
                    #[allow(non_camel_case_types)]
                    struct KillServerSvc<T: StateService>(pub Arc<T>);
                    impl<
                        T: StateService,
                    > tonic::server::UnaryService<super::KillServerRequest>
                    for KillServerSvc<T> {
                        type Response = super::KillServerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KillServerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StateService>::kill_server(&inner, request).await
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
                        let method = KillServerSvc(inner);
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
    impl<T: StateService> Clone for StateServiceServer<T> {
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
    impl<T: StateService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: StateService> tonic::server::NamedService for StateServiceServer<T> {
        const NAME: &'static str = "d2rpc.StateService";
    }
}
