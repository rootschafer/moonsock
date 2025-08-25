use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HelpText(String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GcodeHelp {
	#[serde(flatten)]
	pub handlers: BTreeMap<String, HelpText>,
}


#[cfg(test)]
mod tests {
	use super::*;
	use crate::{response::MoonResultData, JsonRpcVersion, MoonResponse};

	#[test]
	fn test_deserialize_gcode_help() {
		let json = r#"{
            "jsonrpc": "2.0",
            "result": {
                "RESTORE_GCODE_STATE": "Restore a previously saved G-Code state",
                "PID_CALIBRATE": "Run PID calibration test",
                "QUERY_ADC": "Report the last value of an analog pin"
            },
            "id": 345
        }"#;

		let mut handlers = BTreeMap::new();
		handlers.insert(
			"RESTORE_GCODE_STATE".to_string(),
			HelpText("Restore a previously saved G-Code state".to_string()),
		);
		handlers.insert("PID_CALIBRATE".to_string(), HelpText("Run PID calibration test".to_string()));
		handlers.insert(
			"QUERY_ADC".to_string(),
			HelpText("Report the last value of an analog pin".to_string()),
		);

		let expected = MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::GcodeHelp(GcodeHelp { handlers }),
			id: 345,
		};

		let actual: MoonResponse = serde_json::from_str(json).unwrap();
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_serialize_gcode_help() {
		let mut handlers = BTreeMap::new();
		handlers.insert(
			"RESTORE_GCODE_STATE".to_string(),
			HelpText("Restore a previously saved G-Code state".to_string()),
		);
		handlers.insert("PID_CALIBRATE".to_string(), HelpText("Run PID calibration test".to_string()));
		handlers.insert(
			"QUERY_ADC".to_string(),
			HelpText("Report the last value of an analog pin".to_string()),
		);

		let response = MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::GcodeHelp(GcodeHelp { handlers }),
			id: 345,
		};

		let expected = r#"{"jsonrpc":"2.0","result":{"PID_CALIBRATE":"Run PID calibration test","QUERY_ADC":"Report the last value of an analog pin","RESTORE_GCODE_STATE":"Restore a previously saved G-Code state"},"id":345}"#;
		let actual = serde_json::to_string(&response).unwrap();
		assert_eq!(actual, expected);
	}
}

