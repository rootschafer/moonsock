use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GcodeStore {
	pub gcode_store: Vec<GcodeStoreEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GcodeStoreEntry {
	pub message: String,
	pub time: f64,
	#[serde(rename = "type")]
	pub entry_type: GcodeStoreEntryType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GcodeStoreEntryType {
	#[serde(rename = "command")]
	Command,
	#[serde(rename = "response")]
	Response,
}

#[cfg(test)]
mod tests {
	use serde_json;

	use super::*;
	use crate::{response::MoonResultData, JsonRpcVersion, MoonResponse};

	#[test]
	fn test_serialize() {
		let gcode_store = GcodeStore {
			gcode_store: vec![
				GcodeStoreEntry {
					message: "FIRMWARE_RESTART".to_string(),
					time: 1615832299.1167388,
					entry_type: GcodeStoreEntryType::Command,
				},
				GcodeStoreEntry {
					message: "// Klipper state: Ready".to_string(),
					time: 1615832309.9977088,
					entry_type: GcodeStoreEntryType::Response,
				},
			],
		};

		let response = MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::GcodeStore(gcode_store),
			id: 345,
		};

		let expected = r#"{"jsonrpc":"2.0","result":{"gcode_store":[{"message":"FIRMWARE_RESTART","time":1615832299.1167388,"type":"command"},{"message":"// Klipper state: Ready","time":1615832309.9977088,"type":"response"}]},"id":345}"#;
		assert_eq!(serde_json::to_string(&response).unwrap(), expected);
	}

	#[test]
	fn test_deserialize() {
		let input = r#"{"jsonrpc":"2.0","result":{"gcode_store":[{"message":"FIRMWARE_RESTART","time":1615832299.1167388,"type":"command"},{"message":"// Klipper state: Ready","time":1615832309.9977088,"type":"response"}]},"id":345}"#;
		let response: MoonResponse = serde_json::from_str(input).unwrap();

		let gcode_store = GcodeStore {
			gcode_store: vec![
				GcodeStoreEntry {
					message: "FIRMWARE_RESTART".to_string(),
					time: 1615832299.1167388,
					entry_type: GcodeStoreEntryType::Command,
				},
				GcodeStoreEntry {
					message: "// Klipper state: Ready".to_string(),
					time: 1615832309.9977088,
					entry_type: GcodeStoreEntryType::Response,
				},
			],
		};

		let expected = MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::GcodeStore(gcode_store),
			id: 345,
		};

		assert_eq!(response, expected);
	}
}
