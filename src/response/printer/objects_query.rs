use serde::{Deserialize, Serialize};
// use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrinterObjectsQueryResponse {
	pub eventtime: f64,
	pub status: PrinterObjectStatus,
}

// Shared with `notify_status_update` and `printer.objects.query`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PrinterObjectStatus {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gcode_move: Option<GcodeMove>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub toolhead: Option<Toolhead>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub configfile: Option<Configfile>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub extruder: Option<Extruder>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub heater_bed: Option<HeaterBed>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub fan: Option<Fan>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub idle_timeout: Option<IdleTimeout>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub virtual_sdcard: Option<VirtualSdcard>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub print_stats: Option<PrintStats>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub display_status: Option<DisplayStatus>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub z_tilt: Option<ZTilt>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GcodeMove {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub absolute_coordinates: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub absolute_extrude: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub extrude_factor: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gcode_position: Option<Vec<f64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub homing_origin: Option<Vec<f64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub position: Option<Vec<f64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub speed: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub speed_factor: Option<f64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Toolhead {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub homed_axes: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub print_time: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub estimated_print_time: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub extruder: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub position: Option<Vec<f64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_velocity: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_accel: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_accel_to_decel: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub square_corner_velocity: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Configfile {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub config: Option<serde_json::Value>, // Can be more specific if the structure is known
	#[serde(skip_serializing_if = "Option::is_none")]
	pub settings: Option<serde_json::Value>, // Can be more specific if the structure is known
	#[serde(skip_serializing_if = "Option::is_none")]
	pub save_config_pending: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Extruder {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pressure_advance: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub smooth_time: Option<f64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct HeaterBed {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power: Option<f64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Fan {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub speed: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rpm: Option<u32>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct IdleTimeout {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub state: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub printing_time: Option<f64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct VirtualSdcard {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub progress: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_active: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_position: Option<u64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PrintStats {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filename: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub total_duration: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub print_duration: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filament_used: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub state: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub info: Option<PrintStatsInfo>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PrintStatsInfo {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub total_layer: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub current_layer: Option<u32>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DisplayStatus {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub progress: Option<f64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ZTilt {
	pub applied: bool,
}

// impl Default for ZTilt {
// 	fn default() -> Self {
// 		Self { applied: false }
// 	}
// }


#[cfg(test)]
mod tests {
	use serde_json;

	use super::*;
	use crate::{response::MoonResultData, JsonRpcVersion, MoonResponse};

	#[test]
	fn test_deserialize_printer_objects_query_response() {
		let json = r#"{
            "jsonrpc": "2.0",
            "result": {
                "eventtime": 578243.57824499,
                "status": {
                    "gcode_move": {
                        "absolute_coordinates": true,
                        "absolute_extrude": true,
                        "extrude_factor": 1,
                        "gcode_position": [0, 0, 0, 0],
                        "homing_origin": [0, 0, 0, 0],
                        "position": [0, 0, 0, 0],
                        "speed": 1500,
                        "speed_factor": 1
                    },
                    "toolhead": {
                        "position": [0, 0, 0, 0],
                        "status": "Ready"
                    }
                }
            },
            "id": 345
        }"#;

		let expected = MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::PrinterObjectsQueryResponse(PrinterObjectsQueryResponse {
				eventtime: 578243.57824499,
				status: PrinterObjectStatus {
					gcode_move: Some(GcodeMove {
						absolute_coordinates: Some(true),
						absolute_extrude: Some(true),
						extrude_factor: Some(1.0),
						gcode_position: Some(vec![0.0, 0.0, 0.0, 0.0]),
						homing_origin: Some(vec![0.0, 0.0, 0.0, 0.0]),
						position: Some(vec![0.0, 0.0, 0.0, 0.0]),
						speed: Some(1500.0),
						speed_factor: Some(1.0),
					}),
					toolhead: Some(Toolhead {
						position: Some(vec![0.0, 0.0, 0.0, 0.0]),
						status: Some("Ready".to_string()),
						..Default::default()
					}),
					..Default::default()
				},
			}),
			id: 345,
		};

		let actual: MoonResponse = serde_json::from_str(json).unwrap();
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_serialize_printer_objects_query_response() {
		let response = MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::PrinterObjectsQueryResponse(PrinterObjectsQueryResponse {
				eventtime: 578243.57824499,
				status: PrinterObjectStatus {
					gcode_move: Some(GcodeMove {
						absolute_coordinates: Some(true),
						absolute_extrude: Some(true),
						extrude_factor: Some(1.0),
						gcode_position: Some(vec![0.0, 0.0, 0.0, 0.0]),
						homing_origin: Some(vec![0.0, 0.0, 0.0, 0.0]),
						position: Some(vec![0.0, 0.0, 0.0, 0.0]),
						speed: Some(1500.0),
						speed_factor: Some(1.0),
					}),
					toolhead: Some(Toolhead {
						position: Some(vec![0.0, 0.0, 0.0, 0.0]),
						status: Some("Ready".to_string()),
						..Default::default()
					}),
					..Default::default()
				},
			}),
			id: 345,
		};

		let expected = "{\"jsonrpc\":\"2.0\",\"result\":{\"eventtime\":578243.57824499,\"status\":{\"gcode_move\":{\"absolute_coordinates\":true,\"absolute_extrude\":true,\"extrude_factor\":1.0,\"gcode_position\":[0.0,0.0,0.0,0.0],\"homing_origin\":[0.0,0.0,0.0,0.0],\"position\":[0.0,0.0,0.0,0.0],\"speed\":1500.0,\"speed_factor\":1.0},\"toolhead\":{\"position\":[0.0,0.0,0.0,0.0],\"status\":\"Ready\"}}},\"id\":345}";

		let actual = serde_json::to_string(&response).unwrap();
		assert_eq!(actual, expected);
	}
}

