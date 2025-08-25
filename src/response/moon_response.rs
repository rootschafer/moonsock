use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
	jsonrpc_ws_client::{JsonRpcError, JsonRpcResponse},
	JsonRpcVersion, MoonResultData,
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
		error: JsonRpcError,
		#[serde(skip_serializing_if = "Option::is_none")]
		id: Option<u32>,
	},
	// Notification {
	// 	jsonrpc: JsonRpcVersion,
	// 	method: NotificationMethod,
	// 	#[serde(skip_serializing_if = "Option::is_none")]
	// 	params: Option<NotificationParam>,
	// },
}

impl From<JsonRpcResponse> for MoonResponse {
	fn from(response: JsonRpcResponse) -> Self {
		match response {
			JsonRpcResponse::Ok(result) => match serde_json::from_value::<MoonResultData>(result.result) {
				Ok(moon_result) => MoonResponse::MoonResult {
					jsonrpc: JsonRpcVersion::V2,
					result: moon_result,
					id: result.id,
				},
				Err(e) => {
					tracing::error!("Error parsing MoonResultData: {}", e);
					MoonResponse::MoonError {
						jsonrpc: JsonRpcVersion::V2,
						error: JsonRpcError {
							code: 0,
							message: "Failed to parse MoonResultData".to_string(),
							data: None,
						},
						id: Some(result.id),
					}
				}
			},
			JsonRpcResponse::Error(error) => MoonResponse::MoonError {
				jsonrpc: JsonRpcVersion::V2,
				error: error.error,
				id: error.id,
			},
		}
	}
}

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
	// pub fn method(&self) -> Option<&NotificationMethod> {
	// 	match self {
	// 		Self::MoonResult { .. } | Self::MoonError { .. } => None,
	// 		Self::Notification { method, .. } => Some(method),
	// 	}
	// }
	// pub fn params(&self) -> Option<NotificationParam> {
	// 	match self {
	// 		Self::MoonResult { .. } | Self::MoonError { .. } => None,
	// 		Self::Notification { params, .. } => params.clone(),
	// 	}
	// }
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
			} // Self::Notification { .. } => {}
		}
	}
}
