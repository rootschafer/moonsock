// {
//     "jsonrpc": "2.0",
//     "result": {
//         "connection_id": 1730367696
//     },
//     "id": 345
// }
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectionIdResponse {
	pub connection_id: u32,
}


#[cfg(test)]
mod tests {
	use serde_json;

	use super::*;
	use crate::{response::MoonResultData, JsonRpcVersion, MoonResponse};

	#[test]
	fn test_serialize_connection_id() {
		let response = MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::ConnectionIdResponse(ConnectionIdResponse { connection_id: 1730367696 }),
			id: 345,
		};

		let expected = r#"{"jsonrpc":"2.0","result":{"connection_id":1730367696},"id":345}"#;
		assert_eq!(serde_json::to_string(&response).unwrap(), expected);
	}

	#[test]
	fn test_deserialize_connection_id() {
		let input = r#"{"jsonrpc":"2.0","result":{"connection_id":1730367696},"id":345}"#;
		let response: MoonResponse = serde_json::from_str(input).unwrap();

		let expected = MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::ConnectionIdResponse(ConnectionIdResponse { connection_id: 1730367696 }),
			id: 345,
		};

		assert_eq!(response, expected);
	}
}

