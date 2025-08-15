use serde::{Deserialize, Serialize};

pub mod response;
pub use response::*;

mod request;
pub use request::*;

pub mod moon_method;

mod moon_param;

mod notification;
pub use notification::*;

pub mod utils;

// pub mod connection;
// pub use connection::MoonConnection;
pub use moon_method::MoonMethod;
pub use moon_param::*;

// pub mod fast_ws_stuff;
// mod fast_ws_connection;
// pub use fast_ws_connection::*;

pub mod jsonrpc_ws_client;
pub use jsonrpc_ws_client::JsonRpcNotification;

// mod moonraker_client;
// pub use moonraker_client::MoonrakerClient;
// pub use moonraker_client::*;

// pub mod moonraker_client_new;
mod moonraker_client_new;
pub use moonraker_client_new::*;

/// ---------------------- Request Serializing ------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum JsonRpcVersion {
	#[serde(rename = "2.0")]
	#[default]
	V2,
}
