// use jsonrpc_message_derive::JsonRpcMessage;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
	jsonrpc_ws_client::{
		JsonRpcError,
		JsonRpcResponse,
		// JsonRpcMessage
	},
	response::{ServerConfig, ServerInfo},
	JsonRpcVersion, MoonResultData, NotificationMethod, NotificationParam,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoonErrorContent {
	pub code: u32,
	pub message: String,
}

impl fmt::Display for MoonErrorContent {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}: {}", self.code, self.message)
	}
}

impl std::error::Error for MoonErrorContent {}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonRpcMessage)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum MoonResponse {
	MoonResult {
		jsonrpc: JsonRpcVersion,
		result: MoonResultData,
		id: u32,
	},
	MoonError {
		jsonrpc: JsonRpcVersion,
		// error: MoonErrorContent,
		error: JsonRpcError,
		#[serde(skip_serializing_if = "Option::is_none")]
		id: Option<u32>,
	},
	Notification {
		jsonrpc: JsonRpcVersion,
		method: NotificationMethod,
		#[serde(skip_serializing_if = "Option::is_none")]
		params: Option<NotificationParam>,
	},
}

impl From<JsonRpcResponse> for MoonResponse {
	fn from(response: JsonRpcResponse) -> Self {
		match response {
			JsonRpcResponse::Result(result) => MoonResponse::MoonResult {
				jsonrpc: JsonRpcVersion::V2,
				result: serde_json::from_value(result.result).unwrap(),
				id: 0,
			},
			JsonRpcResponse::ReturnedError(error) => MoonResponse::MoonError {
				jsonrpc: JsonRpcVersion::V2,
				// error: serde_json::from_value(error).unwrap(),
				error,
				id: None,
			},
		}
	}
}

// impl JsonRpcMessage for MoonResponse {
//     fn id(&self) -> Option<u32> {
//         match self {
//             MoonResponse::MoonResult { id, .. } => Some(*id),
//             MoonResponse::MoonError { id, .. } => *id,
//             MoonResponse::Notification { .. } => None,
//         }
//     }
//     fn set_id(&mut self, new_id: u32) {
//         match self {
//             MoonResponse::MoonResult { id, .. } => *id = new_id,
//             MoonResponse::MoonError { id, .. } => *id = Some(new_id),
//             MoonResponse::Notification { .. } => {},
//         }
//     }
// }

impl Default for MoonResponse {
	fn default() -> Self {
		MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::None,
			id: 0,
		}
	}
}

impl MoonResponse {
	pub fn method(&self) -> Option<&NotificationMethod> {
		match self {
			Self::MoonResult { .. } | Self::MoonError { .. } => None,
			Self::Notification { method, .. } => Some(method),
		}
	}
	pub fn params(&self) -> Option<NotificationParam> {
		match self {
			Self::MoonResult { .. } | Self::MoonError { .. } => None,
			Self::Notification { params, .. } => params.clone(),
		}
	}
	pub fn set_id(&mut self, new_id: u32) {
		match self {
			Self::MoonError { jsonrpc, error, .. } => {
				let new = Self::MoonError {
					jsonrpc: jsonrpc.clone(),
					error: error.clone(),
					id: Some(new_id),
				};
				*self = new;
			}
			Self::MoonResult { jsonrpc, result, .. } => {
				let new = Self::MoonResult {
					jsonrpc: jsonrpc.clone(),
					result: result.clone(),
					id: new_id,
				};
				*self = new;
			}
			Self::Notification { .. } => {}
		}
	}
	pub fn default_server_info_result(server_info: ServerInfo) -> Self {
		MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::ServerInfo(server_info),
			id: 0,
		}
	}
	pub fn default_server_config_result(config: ServerConfig) -> Self {
		MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::ServerConfig(config),
			id: 0,
		}
	}
	// pub fn new_error(error: MoonErrorContent, id: u32) -> Self {
	//     Self::MoonError {
	//         jsonrpc: JsonRpcVersion::V2,
	//         error,
	//         id,
	//     }
	// }
	// pub fn new_result(result: moon_result::MoonResultData, id: u32) -> Self {
	//     Self::MoonResult {
	//         jsonrpc: JsonRpcVersion::V2,
	//         result,
	//         id,
	//     }
	// }
}

