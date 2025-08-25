// {
//     "jsonrpc": "2.0",
//     "result": {
//         "rolled_over": [
//             "moonraker",
//             "klipper"
//         ],
//         "failed": {}
//     },
//     "id": 345
// }

use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RollOverResponse {
	pub rolled_over: Vec<String>,
	pub failed: HashMap<String, String>,
}


#[cfg(test)]
mod tests {
	use std::collections::HashMap;

	use serde_json;

	use super::*;
	use crate::{response::MoonResultData, JsonRpcVersion, MoonResponse};

	#[test]
	fn test_serialize() {
		let failed = HashMap::new();
		let response = MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::RollOverResponse(RollOverResponse {
				rolled_over: vec!["moonraker".to_string(), "klipper".to_string()],
				failed,
			}),
			id: 345,
		};

		let expected = r#"{"jsonrpc":"2.0","result":{"rolled_over":["moonraker","klipper"],"failed":{}},"id":345}"#;
		assert_eq!(serde_json::to_string(&response).unwrap(), expected);
	}

	#[test]
	fn test_deserialize() {
		let input = r#"{"jsonrpc":"2.0","result":{"rolled_over":["moonraker","klipper"],"failed":{}},"id":345}"#;
		let response: MoonResponse = serde_json::from_str(input).unwrap();

		let failed = HashMap::new();
		let expected = MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::RollOverResponse(RollOverResponse {
				rolled_over: vec!["moonraker".to_string(), "klipper".to_string()],
				failed,
			}),
			id: 345,
		};

		assert_eq!(response, expected);
	}
}
