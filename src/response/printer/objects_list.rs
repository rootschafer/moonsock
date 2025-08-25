use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrinterObjectsListResponse {
	pub objects: Vec<String>, // Array of printer object names
}


#[cfg(test)]
mod tests {
	use super::*;
	use crate::{response::MoonResultData, JsonRpcVersion, MoonResponse};

	#[test]
	fn test_deserialize() {
		let json = r#"{
            "jsonrpc": "2.0",
            "result": {
                "objects": ["gcode", "toolhead", "bed_mesh", "configfile"]
            },
            "id": 345
        }"#;

		let expected = MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::PrinterObjectsListResponse(PrinterObjectsListResponse {
				objects: vec![
					"gcode".to_string(),
					"toolhead".to_string(),
					"bed_mesh".to_string(),
					"configfile".to_string(),
				],
			}),
			id: 345,
		};

		let actual: MoonResponse = serde_json::from_str(json).unwrap();
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_serialize() {
		let response = MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::PrinterObjectsListResponse(PrinterObjectsListResponse {
				objects: vec![
					"gcode".to_string(),
					"toolhead".to_string(),
					"bed_mesh".to_string(),
					"configfile".to_string(),
				],
			}),
			id: 345,
		};

		let expected = "{\"jsonrpc\":\"2.0\",\"result\":{\"objects\":[\"gcode\",\"toolhead\",\"bed_mesh\",\"configfile\"]},\"id\":345}";

		let actual = serde_json::to_string(&response).unwrap();
		assert_eq!(actual, expected);
	}
}

