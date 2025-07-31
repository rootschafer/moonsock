use serde::{Deserialize, Serialize};

/// ```json
/// {
///     "jsonrpc": "2.0",
///     "result": {
///         "x": "TRIGGERED",
///         "y": "open",
///         "z": "open"
///     },
///     "id": 345
/// }
/// ```
/// An object containing the current endstop state, where each field is an endstop identifier, with a string value of "open" or "TRIGGERED".
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EndstopState {
	#[serde(rename = "open")]
	Open,
	#[serde(rename = "TRIGGERED")]
	Triggered,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EndstopStatus {
	pub x: EndstopState,
	pub y: EndstopState,
	pub z: EndstopState,
}


#[cfg(test)]
mod tests {
	use super::*;
	use crate::{response::MoonResultData, JsonRpcVersion, MoonResponse};

	#[test]
	fn test_deserialize_endstop_status() {
		let json = r#"{
            "jsonrpc": "2.0",
            "result": {
                "x": "TRIGGERED",
                "y": "open",
                "z": "open"
            },
            "id": 345
        }"#;

		let expected = MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::EndstopStatus(EndstopStatus {
				x: EndstopState::Triggered,
				y: EndstopState::Open,
				z: EndstopState::Open,
			}),
			id: 345,
		};

		let actual: MoonResponse = serde_json::from_str(json).unwrap();
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_serialize_endstop_status() {
		let response = MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::EndstopStatus(EndstopStatus {
				x: EndstopState::Triggered,
				y: EndstopState::Open,
				z: EndstopState::Open,
			}),
			id: 345,
		};

		let expected =
			"{\"jsonrpc\":\"2.0\",\"result\":{\"x\":\"TRIGGERED\",\"y\":\"open\",\"z\":\"open\"},\"id\":345}";
		let actual = serde_json::to_string(&response).unwrap();
		assert_eq!(actual, expected);
	}
}

