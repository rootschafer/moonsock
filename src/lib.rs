use serde::{Deserialize, Serialize};

pub mod response;
pub use response::*;

mod request;
pub use request::*;

pub mod moon_method;
pub use moon_method::MoonMethod;

mod notification;
pub use notification::*;

pub mod utils;

mod moon_param;
pub use moon_param::*;

pub mod jsonrpc_ws_client;
pub use jsonrpc_ws_client::JsonRpcNotification;

mod moonraker_client;
pub use moonraker_client::*;

/// ---------------------- Request Serializing ------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum JsonRpcVersion {
	#[serde(rename = "2.0")]
	#[default]
	V2,
}
