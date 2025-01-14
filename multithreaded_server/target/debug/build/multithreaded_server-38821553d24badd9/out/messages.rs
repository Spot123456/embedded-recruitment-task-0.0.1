#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoMessage {
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddRequest {
    #[prost(int32, tag = "1")]
    pub a: i32,
    #[prost(int32, tag = "2")]
    pub b: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddResponse {
    #[prost(int32, tag = "1")]
    pub result: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientMessage {
    #[prost(oneof = "client_message::Message", tags = "1, 2")]
    pub message: ::core::option::Option<client_message::Message>,
}
/// Nested message and enum types in `ClientMessage`.
pub mod client_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "1")]
        EchoMessage(super::EchoMessage),
        #[prost(message, tag = "2")]
        AddRequest(super::AddRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerMessage {
    #[prost(oneof = "server_message::Message", tags = "1, 2")]
    pub message: ::core::option::Option<server_message::Message>,
}
/// Nested message and enum types in `ServerMessage`.
pub mod server_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "1")]
        EchoMessage(super::EchoMessage),
        #[prost(message, tag = "2")]
        AddResponse(super::AddResponse),
    }
}
