// @generated
/// AuthzAllowance creates allowance only authz message for a specific grantee
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthzAllowance {
    /// allowance can be any of basic and periodic fee allowance.
    #[prost(message, optional, tag = "1")]
    pub allowance: ::core::option::Option<::pbjson_types::Any>,
    #[prost(string, tag = "2")]
    pub authz_grantee: ::prost::alloc::string::String,
}
/// ContractsAllowance creates allowance only for specific contracts
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractsAllowance {
    /// allowance can be any of basic and periodic fee allowance.
    #[prost(message, optional, tag = "1")]
    pub allowance: ::core::option::Option<::pbjson_types::Any>,
    #[prost(string, repeated, tag = "2")]
    pub contract_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(uint32, tag = "1")]
    pub platform_percentage: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWebAuthNVerifyRegisterRequest {
    #[prost(string, tag = "1")]
    pub addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub challenge: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub rp: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWebAuthNVerifyRegisterResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub credential: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWebAuthNVerifyAuthenticateRequest {
    #[prost(string, tag = "1")]
    pub addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub challenge: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub rp: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub credential: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWebAuthNVerifyAuthenticateResponse {}
/// MsgSend represents a message to send coins from one account to another.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSend {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSendResponse defines the Msg/Send response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendResponse {}
/// MsgMultiSend represents an arbitrary multi-in, multi-out send message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMultiSend {
    /// Inputs, despite being `repeated`, only allows one sender input. This is
    /// checked in MsgMultiSend's ValidateBasic.
    #[prost(message, repeated, tag = "1")]
    pub inputs: ::prost::alloc::vec::Vec<super::super::cosmos::bank::v1beta1::Input>,
    #[prost(message, repeated, tag = "2")]
    pub outputs: ::prost::alloc::vec::Vec<super::super::cosmos::bank::v1beta1::Output>,
}
/// MsgMultiSendResponse defines the Msg/MultiSend response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMultiSendResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetPlatformPercentage {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// platform_percentage is the platform fee percentage to multiplied by 10000
    #[prost(uint32, tag = "2")]
    pub platform_percentage: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetPlatformPercentageResponse {}
include!("xion.v1.serde.rs");
include!("xion.v1.tonic.rs");
// @@protoc_insertion_point(module)
