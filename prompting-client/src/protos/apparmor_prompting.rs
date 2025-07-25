// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromptReply {
    #[prost(string, tag = "1")]
    pub prompt_id: ::prost::alloc::string::String,
    #[prost(enumeration = "Action", tag = "2")]
    pub action: i32,
    #[prost(enumeration = "Lifespan", tag = "3")]
    pub lifespan: i32,
    #[prost(oneof = "prompt_reply::PromptReply", tags = "4")]
    pub prompt_reply: ::core::option::Option<prompt_reply::PromptReply>,
}
/// Nested message and enum types in `PromptReply`.
pub mod prompt_reply {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PromptReply {
        #[prost(message, tag = "4")]
        HomePromptReply(super::HomePromptReply),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromptReplyResponse {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    #[prost(
        oneof = "prompt_reply_response::PromptReplyType",
        tags = "2, 3, 4, 5, 6, 7, 8, 9, 10"
    )]
    pub prompt_reply_type: ::core::option::Option<
        prompt_reply_response::PromptReplyType,
    >,
}
/// Nested message and enum types in `PromptReplyResponse`.
pub mod prompt_reply_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HomeRuleConflicts {
        #[prost(message, repeated, tag = "1")]
        pub conflicts: ::prost::alloc::vec::Vec<HomeRuleConflict>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HomeRuleConflict {
        #[prost(enumeration = "super::HomePermission", tag = "1")]
        pub permission: i32,
        #[prost(string, tag = "2")]
        pub variant: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub conflicting_id: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InvalidHomePermissions {
        #[prost(enumeration = "super::HomePermission", repeated, tag = "1")]
        pub requested: ::prost::alloc::vec::Vec<i32>,
        #[prost(enumeration = "super::HomePermission", repeated, tag = "2")]
        pub replied: ::prost::alloc::vec::Vec<i32>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InvalidPathPattern {
        #[prost(string, tag = "1")]
        pub requested: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub replied: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ParseError {
        #[prost(string, tag = "1")]
        pub field: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnsupportedValue {
        #[prost(string, tag = "1")]
        pub field: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "2")]
        pub supported: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "3")]
        pub provided: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PromptReplyType {
        #[prost(message, tag = "2")]
        Success(()),
        #[prost(message, tag = "3")]
        Raw(()),
        #[prost(message, tag = "4")]
        PromptNotFound(()),
        #[prost(message, tag = "5")]
        RuleNotFound(()),
        #[prost(message, tag = "6")]
        RuleConflicts(HomeRuleConflicts),
        #[prost(message, tag = "7")]
        InvalidPermissions(InvalidHomePermissions),
        #[prost(message, tag = "8")]
        InvalidPathPattern(InvalidPathPattern),
        #[prost(message, tag = "9")]
        ParseError(ParseError),
        #[prost(message, tag = "10")]
        UnsupportedValue(UnsupportedValue),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentPromptResponse {
    #[prost(oneof = "get_current_prompt_response::Prompt", tags = "1")]
    pub prompt: ::core::option::Option<get_current_prompt_response::Prompt>,
}
/// Nested message and enum types in `GetCurrentPromptResponse`.
pub mod get_current_prompt_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Prompt {
        #[prost(message, tag = "1")]
        HomePrompt(super::HomePrompt),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HomePromptReply {
    #[prost(string, tag = "1")]
    pub path_pattern: ::prost::alloc::string::String,
    #[prost(enumeration = "HomePermission", repeated, tag = "2")]
    pub permissions: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HomePrompt {
    #[prost(message, optional, tag = "1")]
    pub meta_data: ::core::option::Option<MetaData>,
    #[prost(string, tag = "2")]
    pub requested_path: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub home_dir: ::prost::alloc::string::String,
    #[prost(enumeration = "HomePermission", repeated, tag = "4")]
    pub requested_permissions: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration = "HomePermission", repeated, tag = "5")]
    pub available_permissions: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration = "HomePermission", repeated, tag = "6")]
    pub suggested_permissions: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "7")]
    pub pattern_options: ::prost::alloc::vec::Vec<home_prompt::PatternOption>,
    #[prost(int32, tag = "8")]
    pub initial_pattern_option: i32,
    #[prost(message, optional, tag = "9")]
    pub enriched_path_kind: ::core::option::Option<EnrichedPathKind>,
}
/// Nested message and enum types in `HomePrompt`.
pub mod home_prompt {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PatternOption {
        #[prost(enumeration = "super::HomePatternType", tag = "1")]
        pub home_pattern_type: i32,
        #[prost(string, tag = "2")]
        pub path_pattern: ::prost::alloc::string::String,
        #[prost(bool, tag = "3")]
        pub show_initially: bool,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaData {
    #[prost(string, tag = "1")]
    pub prompt_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub snap_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub store_url: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub publisher: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub updated_at: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ResolveHomePatternTypeResponse {
    #[prost(enumeration = "HomePatternType", tag = "1")]
    pub home_pattern_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLoggingFilterResponse {
    #[prost(string, tag = "1")]
    pub current: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnrichedPathKind {
    #[prost(oneof = "enriched_path_kind::Kind", tags = "1, 2, 3, 4, 5, 6")]
    pub kind: ::core::option::Option<enriched_path_kind::Kind>,
}
/// Nested message and enum types in `EnrichedPathKind`.
pub mod enriched_path_kind {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        HomeDir(super::HomeDir),
        #[prost(message, tag = "2")]
        TopLevelDir(super::TopLevelDir),
        #[prost(message, tag = "3")]
        SubDir(super::SubDir),
        #[prost(message, tag = "4")]
        HomeDirFile(super::HomeDirFile),
        #[prost(message, tag = "5")]
        TopLevelDirFile(super::TopLevelDirFile),
        #[prost(message, tag = "6")]
        SubDirFile(super::SubDirFile),
    }
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HomeDir {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopLevelDir {
    #[prost(string, tag = "1")]
    pub dirname: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SubDir {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HomeDirFile {
    #[prost(string, tag = "1")]
    pub filename: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopLevelDirFile {
    #[prost(string, tag = "1")]
    pub dirname: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub filename: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SubDirFile {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    Allow = 0,
    Deny = 1,
}
impl Action {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Allow => "ALLOW",
            Self::Deny => "DENY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ALLOW" => Some(Self::Allow),
            "DENY" => Some(Self::Deny),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Lifespan {
    Single = 0,
    Session = 1,
    Forever = 2,
}
impl Lifespan {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Single => "SINGLE",
            Self::Session => "SESSION",
            Self::Forever => "FOREVER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SINGLE" => Some(Self::Single),
            "SESSION" => Some(Self::Session),
            "FOREVER" => Some(Self::Forever),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HomePermission {
    Read = 0,
    Write = 1,
    Execute = 2,
}
impl HomePermission {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Read => "READ",
            Self::Write => "WRITE",
            Self::Execute => "EXECUTE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "READ" => Some(Self::Read),
            "WRITE" => Some(Self::Write),
            "EXECUTE" => Some(Self::Execute),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HomePatternType {
    RequestedDirectory = 0,
    RequestedFile = 1,
    TopLevelDirectory = 2,
    ContainingDirectory = 3,
    HomeDirectory = 4,
    MatchingFileExtension = 5,
    RequestedDirectoryContents = 6,
}
impl HomePatternType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::RequestedDirectory => "REQUESTED_DIRECTORY",
            Self::RequestedFile => "REQUESTED_FILE",
            Self::TopLevelDirectory => "TOP_LEVEL_DIRECTORY",
            Self::ContainingDirectory => "CONTAINING_DIRECTORY",
            Self::HomeDirectory => "HOME_DIRECTORY",
            Self::MatchingFileExtension => "MATCHING_FILE_EXTENSION",
            Self::RequestedDirectoryContents => "REQUESTED_DIRECTORY_CONTENTS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REQUESTED_DIRECTORY" => Some(Self::RequestedDirectory),
            "REQUESTED_FILE" => Some(Self::RequestedFile),
            "TOP_LEVEL_DIRECTORY" => Some(Self::TopLevelDirectory),
            "CONTAINING_DIRECTORY" => Some(Self::ContainingDirectory),
            "HOME_DIRECTORY" => Some(Self::HomeDirectory),
            "MATCHING_FILE_EXTENSION" => Some(Self::MatchingFileExtension),
            "REQUESTED_DIRECTORY_CONTENTS" => Some(Self::RequestedDirectoryContents),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod app_armor_prompting_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AppArmorPromptingClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AppArmorPromptingClient<tonic::transport::Channel> {
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
    impl<T> AppArmorPromptingClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> AppArmorPromptingClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            AppArmorPromptingClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_current_prompt(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetCurrentPromptResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apparmor_prompting.AppArmorPrompting/GetCurrentPrompt",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "apparmor_prompting.AppArmorPrompting",
                        "GetCurrentPrompt",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reply_to_prompt(
            &mut self,
            request: impl tonic::IntoRequest<super::PromptReply>,
        ) -> std::result::Result<
            tonic::Response<super::PromptReplyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apparmor_prompting.AppArmorPrompting/ReplyToPrompt",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "apparmor_prompting.AppArmorPrompting",
                        "ReplyToPrompt",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn resolve_home_pattern_type(
            &mut self,
            request: impl tonic::IntoRequest<::prost::alloc::string::String>,
        ) -> std::result::Result<
            tonic::Response<super::ResolveHomePatternTypeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apparmor_prompting.AppArmorPrompting/ResolveHomePatternType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "apparmor_prompting.AppArmorPrompting",
                        "ResolveHomePatternType",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_logging_filter(
            &mut self,
            request: impl tonic::IntoRequest<::prost::alloc::string::String>,
        ) -> std::result::Result<
            tonic::Response<super::SetLoggingFilterResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apparmor_prompting.AppArmorPrompting/SetLoggingFilter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "apparmor_prompting.AppArmorPrompting",
                        "SetLoggingFilter",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod app_armor_prompting_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AppArmorPromptingServer.
    #[async_trait]
    pub trait AppArmorPrompting: std::marker::Send + std::marker::Sync + 'static {
        async fn get_current_prompt(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetCurrentPromptResponse>,
            tonic::Status,
        >;
        async fn reply_to_prompt(
            &self,
            request: tonic::Request<super::PromptReply>,
        ) -> std::result::Result<
            tonic::Response<super::PromptReplyResponse>,
            tonic::Status,
        >;
        async fn resolve_home_pattern_type(
            &self,
            request: tonic::Request<::prost::alloc::string::String>,
        ) -> std::result::Result<
            tonic::Response<super::ResolveHomePatternTypeResponse>,
            tonic::Status,
        >;
        async fn set_logging_filter(
            &self,
            request: tonic::Request<::prost::alloc::string::String>,
        ) -> std::result::Result<
            tonic::Response<super::SetLoggingFilterResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct AppArmorPromptingServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> AppArmorPromptingServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AppArmorPromptingServer<T>
    where
        T: AppArmorPrompting,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/apparmor_prompting.AppArmorPrompting/GetCurrentPrompt" => {
                    #[allow(non_camel_case_types)]
                    struct GetCurrentPromptSvc<T: AppArmorPrompting>(pub Arc<T>);
                    impl<T: AppArmorPrompting> tonic::server::UnaryService<()>
                    for GetCurrentPromptSvc<T> {
                        type Response = super::GetCurrentPromptResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AppArmorPrompting>::get_current_prompt(
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
                        let method = GetCurrentPromptSvc(inner);
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
                "/apparmor_prompting.AppArmorPrompting/ReplyToPrompt" => {
                    #[allow(non_camel_case_types)]
                    struct ReplyToPromptSvc<T: AppArmorPrompting>(pub Arc<T>);
                    impl<
                        T: AppArmorPrompting,
                    > tonic::server::UnaryService<super::PromptReply>
                    for ReplyToPromptSvc<T> {
                        type Response = super::PromptReplyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PromptReply>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AppArmorPrompting>::reply_to_prompt(&inner, request)
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
                        let method = ReplyToPromptSvc(inner);
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
                "/apparmor_prompting.AppArmorPrompting/ResolveHomePatternType" => {
                    #[allow(non_camel_case_types)]
                    struct ResolveHomePatternTypeSvc<T: AppArmorPrompting>(pub Arc<T>);
                    impl<
                        T: AppArmorPrompting,
                    > tonic::server::UnaryService<::prost::alloc::string::String>
                    for ResolveHomePatternTypeSvc<T> {
                        type Response = super::ResolveHomePatternTypeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<::prost::alloc::string::String>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AppArmorPrompting>::resolve_home_pattern_type(
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
                        let method = ResolveHomePatternTypeSvc(inner);
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
                "/apparmor_prompting.AppArmorPrompting/SetLoggingFilter" => {
                    #[allow(non_camel_case_types)]
                    struct SetLoggingFilterSvc<T: AppArmorPrompting>(pub Arc<T>);
                    impl<
                        T: AppArmorPrompting,
                    > tonic::server::UnaryService<::prost::alloc::string::String>
                    for SetLoggingFilterSvc<T> {
                        type Response = super::SetLoggingFilterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<::prost::alloc::string::String>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AppArmorPrompting>::set_logging_filter(
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
                        let method = SetLoggingFilterSvc(inner);
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
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for AppArmorPromptingServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "apparmor_prompting.AppArmorPrompting";
    impl<T> tonic::server::NamedService for AppArmorPromptingServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
