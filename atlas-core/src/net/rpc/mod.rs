pub mod server;
pub mod client;
pub mod codec_json;
pub mod codec_rmp;

pub mod packet;
pub mod router;
pub mod router_spec;


// 公共 re-export，方便其他地方统一使用 Codec
#[cfg(feature = "json_codec")]
pub use codec_json::JsonCodec as Codec;

#[cfg(feature = "msgpack_codec")]
pub use codec_rmp::MsgPackCodec as Codec;