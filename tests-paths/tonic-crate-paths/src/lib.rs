#![allow(dead_code, non_upper_case_globals)]
/// pub enum `Code`
pub const Code: crate_paths::Path = crate_paths::Path::new("tonic::Code");

/// pub struct `ConnectError`
pub const ConnectError: crate_paths::Path = crate_paths::Path::new("tonic::ConnectError");

/// pub struct `Extensions`
pub const Extensions: crate_paths::Path = crate_paths::Path::new("tonic::Extensions");

/// pub struct `GrpcMethod`
pub const GrpcMethod: crate_paths::Path = crate_paths::Path::new("tonic::GrpcMethod");

/// pub trait `IntoRequest`
pub const IntoRequest: crate_paths::Path = crate_paths::Path::new("tonic::IntoRequest");

/// pub trait `IntoStreamingRequest`
pub const IntoStreamingRequest: crate_paths::Path =
	crate_paths::Path::new("tonic::IntoStreamingRequest");

/// pub struct `Request`
pub const Request: crate_paths::Path = crate_paths::Path::new("tonic::Request");

/// pub struct `Response`
pub const Response: crate_paths::Path = crate_paths::Path::new("tonic::Response");

/// pub type alias `Result`
pub const Result: crate_paths::Path = crate_paths::Path::new("tonic::Result");

/// pub struct `Status`
pub const Status: crate_paths::Path = crate_paths::Path::new("tonic::Status");

/// pub struct `Streaming`
pub const Streaming: crate_paths::Path = crate_paths::Path::new("tonic::Streaming");

/// pub struct `TimeoutExpired`
pub const TimeoutExpired: crate_paths::Path = crate_paths::Path::new("tonic::TimeoutExpired");

/// pub proc attribute `async_trait`
pub const async_trait: crate_paths::Path = crate_paths::Path::new("tonic::async_trait");

/// pub macro `include_file_descriptor_set`
pub const include_file_descriptor_set: crate_paths::Path =
	crate_paths::Path::new("tonic::include_file_descriptor_set");

/// pub macro `include_proto`
pub const include_proto: crate_paths::Path = crate_paths::Path::new("tonic::include_proto");
pub mod body {
	/// pub struct `Body`
	pub const Body: crate_paths::Path = crate_paths::Path::new("tonic::body::Body");
}
pub mod client {
	/// pub struct `Grpc`
	pub const Grpc: crate_paths::Path = crate_paths::Path::new("tonic::client::Grpc");

	/// pub trait `GrpcService`
	pub const GrpcService: crate_paths::Path = crate_paths::Path::new("tonic::client::GrpcService");
}
pub mod codec {
	/// pub struct `BufferSettings`
	pub const BufferSettings: crate_paths::Path =
		crate_paths::Path::new("tonic::codec::BufferSettings");

	/// pub trait `Codec`
	pub const Codec: crate_paths::Path = crate_paths::Path::new("tonic::codec::Codec");

	/// pub enum `CompressionEncoding`
	pub const CompressionEncoding: crate_paths::Path =
		crate_paths::Path::new("tonic::codec::CompressionEncoding");

	/// pub struct `DecodeBuf`
	pub const DecodeBuf: crate_paths::Path = crate_paths::Path::new("tonic::codec::DecodeBuf");

	/// pub trait `Decoder`
	pub const Decoder: crate_paths::Path = crate_paths::Path::new("tonic::codec::Decoder");

	/// pub struct `EnabledCompressionEncodings`
	pub const EnabledCompressionEncodings: crate_paths::Path =
		crate_paths::Path::new("tonic::codec::EnabledCompressionEncodings");

	/// pub struct `EncodeBody`
	pub const EncodeBody: crate_paths::Path = crate_paths::Path::new("tonic::codec::EncodeBody");

	/// pub struct `EncodeBuf`
	pub const EncodeBuf: crate_paths::Path = crate_paths::Path::new("tonic::codec::EncodeBuf");

	/// pub trait `Encoder`
	pub const Encoder: crate_paths::Path = crate_paths::Path::new("tonic::codec::Encoder");

	/// pub struct `ProstCodec`
	pub const ProstCodec: crate_paths::Path = crate_paths::Path::new("tonic::codec::ProstCodec");

	/// pub struct `Streaming`
	pub const Streaming: crate_paths::Path = crate_paths::Path::new("tonic::codec::Streaming");
}
pub mod metadata {
	/// pub enum `Ascii`
	pub const Ascii: crate_paths::Path = crate_paths::Path::new("tonic::metadata::Ascii");

	/// pub type alias `AsciiMetadataKey`
	pub const AsciiMetadataKey: crate_paths::Path =
		crate_paths::Path::new("tonic::metadata::AsciiMetadataKey");

	/// pub type alias `AsciiMetadataValue`
	pub const AsciiMetadataValue: crate_paths::Path =
		crate_paths::Path::new("tonic::metadata::AsciiMetadataValue");

	/// pub enum `Binary`
	pub const Binary: crate_paths::Path = crate_paths::Path::new("tonic::metadata::Binary");

	/// pub type alias `BinaryMetadataKey`
	pub const BinaryMetadataKey: crate_paths::Path =
		crate_paths::Path::new("tonic::metadata::BinaryMetadataKey");

	/// pub type alias `BinaryMetadataValue`
	pub const BinaryMetadataValue: crate_paths::Path =
		crate_paths::Path::new("tonic::metadata::BinaryMetadataValue");

	/// pub enum `Entry`
	pub const Entry: crate_paths::Path = crate_paths::Path::new("tonic::metadata::Entry");

	/// pub constant `GRPC_CONTENT_TYPE`
	pub const GRPC_CONTENT_TYPE: crate_paths::Path =
		crate_paths::Path::new("tonic::metadata::GRPC_CONTENT_TYPE");

	/// pub struct `GetAll`
	pub const GetAll: crate_paths::Path = crate_paths::Path::new("tonic::metadata::GetAll");

	/// pub struct `Iter`
	pub const Iter: crate_paths::Path = crate_paths::Path::new("tonic::metadata::Iter");

	/// pub struct `IterMut`
	pub const IterMut: crate_paths::Path = crate_paths::Path::new("tonic::metadata::IterMut");

	/// pub enum `KeyAndMutValueRef`
	pub const KeyAndMutValueRef: crate_paths::Path =
		crate_paths::Path::new("tonic::metadata::KeyAndMutValueRef");

	/// pub enum `KeyAndValueRef`
	pub const KeyAndValueRef: crate_paths::Path =
		crate_paths::Path::new("tonic::metadata::KeyAndValueRef");

	/// pub enum `KeyRef`
	pub const KeyRef: crate_paths::Path = crate_paths::Path::new("tonic::metadata::KeyRef");

	/// pub struct `Keys`
	pub const Keys: crate_paths::Path = crate_paths::Path::new("tonic::metadata::Keys");

	/// pub struct `MetadataKey`
	pub const MetadataKey: crate_paths::Path =
		crate_paths::Path::new("tonic::metadata::MetadataKey");

	/// pub struct `MetadataMap`
	pub const MetadataMap: crate_paths::Path =
		crate_paths::Path::new("tonic::metadata::MetadataMap");

	/// pub struct `MetadataValue`
	pub const MetadataValue: crate_paths::Path =
		crate_paths::Path::new("tonic::metadata::MetadataValue");

	/// pub struct `OccupiedEntry`
	pub const OccupiedEntry: crate_paths::Path =
		crate_paths::Path::new("tonic::metadata::OccupiedEntry");

	/// pub struct `VacantEntry`
	pub const VacantEntry: crate_paths::Path =
		crate_paths::Path::new("tonic::metadata::VacantEntry");

	/// pub struct `ValueDrain`
	pub const ValueDrain: crate_paths::Path = crate_paths::Path::new("tonic::metadata::ValueDrain");

	/// pub struct `ValueIter`
	pub const ValueIter: crate_paths::Path = crate_paths::Path::new("tonic::metadata::ValueIter");

	/// pub enum `ValueRef`
	pub const ValueRef: crate_paths::Path = crate_paths::Path::new("tonic::metadata::ValueRef");

	/// pub enum `ValueRefMut`
	pub const ValueRefMut: crate_paths::Path =
		crate_paths::Path::new("tonic::metadata::ValueRefMut");

	/// pub struct `Values`
	pub const Values: crate_paths::Path = crate_paths::Path::new("tonic::metadata::Values");

	/// pub struct `ValuesMut`
	pub const ValuesMut: crate_paths::Path = crate_paths::Path::new("tonic::metadata::ValuesMut");
	pub mod errors {
		/// pub struct `InvalidMetadataKey`
		pub const InvalidMetadataKey: crate_paths::Path =
			crate_paths::Path::new("tonic::metadata::errors::InvalidMetadataKey");

		/// pub struct `InvalidMetadataValue`
		pub const InvalidMetadataValue: crate_paths::Path =
			crate_paths::Path::new("tonic::metadata::errors::InvalidMetadataValue");

		/// pub struct `InvalidMetadataValueBytes`
		pub const InvalidMetadataValueBytes: crate_paths::Path =
			crate_paths::Path::new("tonic::metadata::errors::InvalidMetadataValueBytes");

		/// pub struct `ToStrError`
		pub const ToStrError: crate_paths::Path =
			crate_paths::Path::new("tonic::metadata::errors::ToStrError");
	}
}
pub mod server {
	/// pub trait `ClientStreamingService`
	pub const ClientStreamingService: crate_paths::Path =
		crate_paths::Path::new("tonic::server::ClientStreamingService");

	/// pub struct `Grpc`
	pub const Grpc: crate_paths::Path = crate_paths::Path::new("tonic::server::Grpc");

	/// pub trait `NamedService`
	pub const NamedService: crate_paths::Path =
		crate_paths::Path::new("tonic::server::NamedService");

	/// pub trait `ServerStreamingService`
	pub const ServerStreamingService: crate_paths::Path =
		crate_paths::Path::new("tonic::server::ServerStreamingService");

	/// pub trait `StreamingService`
	pub const StreamingService: crate_paths::Path =
		crate_paths::Path::new("tonic::server::StreamingService");

	/// pub trait `UnaryService`
	pub const UnaryService: crate_paths::Path =
		crate_paths::Path::new("tonic::server::UnaryService");
}
pub mod service {
	/// pub struct `AxumBody`
	pub const AxumBody: crate_paths::Path = crate_paths::Path::new("tonic::service::AxumBody");

	/// pub struct `AxumRouter`
	pub const AxumRouter: crate_paths::Path = crate_paths::Path::new("tonic::service::AxumRouter");

	/// pub trait `Interceptor`
	pub const Interceptor: crate_paths::Path =
		crate_paths::Path::new("tonic::service::Interceptor");

	/// pub struct `InterceptorLayer`
	pub const InterceptorLayer: crate_paths::Path =
		crate_paths::Path::new("tonic::service::InterceptorLayer");

	/// pub trait `LayerExt`
	pub const LayerExt: crate_paths::Path = crate_paths::Path::new("tonic::service::LayerExt");

	/// pub struct `Layered`
	pub const Layered: crate_paths::Path = crate_paths::Path::new("tonic::service::Layered");

	/// pub struct `Routes`
	pub const Routes: crate_paths::Path = crate_paths::Path::new("tonic::service::Routes");

	/// pub struct `RoutesBuilder`
	pub const RoutesBuilder: crate_paths::Path =
		crate_paths::Path::new("tonic::service::RoutesBuilder");
	pub mod interceptor {
		/// pub struct `InterceptedService`
		pub const InterceptedService: crate_paths::Path =
			crate_paths::Path::new("tonic::service::interceptor::InterceptedService");

		/// pub trait `Interceptor`
		pub const Interceptor: crate_paths::Path =
			crate_paths::Path::new("tonic::service::interceptor::Interceptor");

		/// pub struct `InterceptorLayer`
		pub const InterceptorLayer: crate_paths::Path =
			crate_paths::Path::new("tonic::service::interceptor::InterceptorLayer");

		/// pub struct `ResponseBody`
		pub const ResponseBody: crate_paths::Path =
			crate_paths::Path::new("tonic::service::interceptor::ResponseBody");

		/// pub struct `ResponseFuture`
		pub const ResponseFuture: crate_paths::Path =
			crate_paths::Path::new("tonic::service::interceptor::ResponseFuture");
	}
	pub mod recover_error {
		/// pub struct `RecoverError`
		pub const RecoverError: crate_paths::Path =
			crate_paths::Path::new("tonic::service::recover_error::RecoverError");

		/// pub struct `RecoverErrorLayer`
		pub const RecoverErrorLayer: crate_paths::Path =
			crate_paths::Path::new("tonic::service::recover_error::RecoverErrorLayer");

		/// pub struct `ResponseBody`
		pub const ResponseBody: crate_paths::Path =
			crate_paths::Path::new("tonic::service::recover_error::ResponseBody");

		/// pub struct `ResponseFuture`
		pub const ResponseFuture: crate_paths::Path =
			crate_paths::Path::new("tonic::service::recover_error::ResponseFuture");
	}
}
pub mod transport {
	/// pub trait `Body`
	pub const Body: crate_paths::Path = crate_paths::Path::new("tonic::transport::Body");

	/// pub struct `Certificate`
	pub const Certificate: crate_paths::Path =
		crate_paths::Path::new("tonic::transport::Certificate");

	/// pub struct `CertificateDer`
	pub const CertificateDer: crate_paths::Path =
		crate_paths::Path::new("tonic::transport::CertificateDer");

	/// pub struct `Channel`
	pub const Channel: crate_paths::Path = crate_paths::Path::new("tonic::transport::Channel");

	/// pub struct `Endpoint`
	pub const Endpoint: crate_paths::Path = crate_paths::Path::new("tonic::transport::Endpoint");

	/// pub struct `Error`
	pub const Error: crate_paths::Path = crate_paths::Path::new("tonic::transport::Error");

	/// pub struct `Identity`
	pub const Identity: crate_paths::Path = crate_paths::Path::new("tonic::transport::Identity");

	/// pub struct `Server`
	pub const Server: crate_paths::Path = crate_paths::Path::new("tonic::transport::Server");

	/// pub struct `Uri`
	pub const Uri: crate_paths::Path = crate_paths::Path::new("tonic::transport::Uri");
	pub mod channel {
		/// pub enum `Change`
		pub const Change: crate_paths::Path =
			crate_paths::Path::new("tonic::transport::channel::Change");

		/// pub struct `Channel`
		pub const Channel: crate_paths::Path =
			crate_paths::Path::new("tonic::transport::channel::Channel");

		/// pub struct `ClientTlsConfig`
		pub const ClientTlsConfig: crate_paths::Path =
			crate_paths::Path::new("tonic::transport::channel::ClientTlsConfig");

		/// pub struct `Endpoint`
		pub const Endpoint: crate_paths::Path =
			crate_paths::Path::new("tonic::transport::channel::Endpoint");

		/// pub struct `ResponseFuture`
		pub const ResponseFuture: crate_paths::Path =
			crate_paths::Path::new("tonic::transport::channel::ResponseFuture");
	}
	pub mod server {
		/// pub trait `Connected`
		pub const Connected: crate_paths::Path =
			crate_paths::Path::new("tonic::transport::server::Connected");

		/// pub struct `Router`
		pub const Router: crate_paths::Path =
			crate_paths::Path::new("tonic::transport::server::Router");

		/// pub struct `Server`
		pub const Server: crate_paths::Path =
			crate_paths::Path::new("tonic::transport::server::Server");

		/// pub struct `ServerTlsConfig`
		pub const ServerTlsConfig: crate_paths::Path =
			crate_paths::Path::new("tonic::transport::server::ServerTlsConfig");

		/// pub struct `TcpConnectInfo`
		pub const TcpConnectInfo: crate_paths::Path =
			crate_paths::Path::new("tonic::transport::server::TcpConnectInfo");

		/// pub struct `TcpIncoming`
		pub const TcpIncoming: crate_paths::Path =
			crate_paths::Path::new("tonic::transport::server::TcpIncoming");

		/// pub struct `TlsConnectInfo`
		pub const TlsConnectInfo: crate_paths::Path =
			crate_paths::Path::new("tonic::transport::server::TlsConnectInfo");

		/// pub struct `UdsConnectInfo`
		pub const UdsConnectInfo: crate_paths::Path =
			crate_paths::Path::new("tonic::transport::server::UdsConnectInfo");
	}
}
