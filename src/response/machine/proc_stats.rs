// {
//     "jsonrpc": "2.0",
//     "result": {
//         "moonraker_stats": [
//             {
//                 "time": 1626612666.850755,
//                 "cpu_usage": 2.66,
//                 "memory": 24732,
//                 "mem_units": "kB"
//             },
//             {
//                 "time": 1626612667.8521338,
//                 "cpu_usage": 2.62,
//                 "memory": 24732,
//                 "mem_units": "kB"
//             }
//         ],
//         "throttled_state": {
//             "bits": 0,
//             "flags": []
//         },
//         "cpu_temp": 45.622,
//         "network": {
//             "lo": {
//                 "rx_bytes": 113516429,
//                 "tx_bytes": 113516429,
//                 "bandwidth": 3342.68
//             },
//             "wlan0": {
//                 "rx_bytes": 48471767,
//                 "tx_bytes": 113430843,
//                 "bandwidth": 4455.91
//             }
//         },
//         "system_cpu_usage": {
//             "cpu": 2.53,
//             "cpu0": 3.03,
//             "cpu1": 5.1,
//             "cpu2": 1.02,
//             "cpu3": 1
//         },
//         "system_uptime": 2876970.38089603,
//         "websocket_connections": 4
//     },
//     "id": 345
// }
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MachineProcStats {
	pub moonraker_stats: Vec<MoonrakerStats>,
	pub throttled_state: CpuThrottledState,
	pub cpu_temp: Option<f64>,
	pub network: BTreeMap<String, NetworkData>,
	pub system_cpu_usage: BTreeMap<String, f64>,
	pub system_uptime: Option<f64>,
	pub websocket_connections: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoonrakerStats {
	pub time: f64,
	pub cpu_usage: f64,
	pub memory: u64,
	pub mem_units: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CpuThrottledState {
	pub bits: u64,
	pub flags: Vec<String>,
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct Network {
//     pub lo: NetworkData,
//     pub eth0: NetworkData,
//     pub wlan0: NetworkData,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct SystemMemory {
//     pub total: u64,
//     pub available: u64,
//     pub used: u64,
// }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkData {
	pub rx_bytes: u64,
	pub tx_bytes: u64,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rx_packets: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tx_packets: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rx_errs: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tx_errs: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rx_drop: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tx_drop: Option<u64>,
	pub bandwidth: f64,
}

impl Default for NetworkData {
	fn default() -> Self {
		NetworkData {
			rx_bytes: 0,
			tx_bytes: 0,
			rx_packets: None,
			tx_packets: None,
			rx_errs: None,
			tx_errs: None,
			rx_drop: None,
			tx_drop: None,
			bandwidth: 0.0,
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		response::{MoonResponse, MoonResultData},
		JsonRpcVersion,
	};

	#[test]
	fn test_deserialize_machine_proc_stats() {
		let json = r#"
            {
                "jsonrpc": "2.0",
                "result": {
                    "moonraker_stats": [
                        {
                            "time": 1626612666.850755,
                            "cpu_usage": 2.66,
                            "memory": 24732,
                            "mem_units": "kB"
                        },
                        {
                            "time": 1626612667.8521338,
                            "cpu_usage": 2.62,
                            "memory": 24732,
                            "mem_units": "kB"
                        }
                    ],
                    "throttled_state": {
                        "bits": 0,
                        "flags": []
                    },
                    "cpu_temp": 45.622,
                    "network": {
                        "lo": {
                            "rx_bytes": 113516429,
                            "tx_bytes": 113516429,
                            "bandwidth": 3342.68
                        },
                        "wlan0": {
                            "rx_bytes": 48471767,
                            "tx_bytes": 113430843,
                            "bandwidth": 4455.91
                        }
                    },
                    "system_cpu_usage": {
                        "cpu": 2.53,
                        "cpu0": 3.03,
                        "cpu1": 5.1,
                        "cpu2": 1.02,
                        "cpu3": 1
                    },
                    "system_uptime": 2876970.38089603,
                    "websocket_connections": 4
                },
                "id": 345
            }
        "#;

		let response: MoonResponse = serde_json::from_str(json).unwrap();

		match response {
			MoonResponse::MoonResult { result, id, .. } => {
				assert_eq!(id, 345);
				match result {
					MoonResultData::MachineProcStats(machine_proc_stats) => {
						assert_eq!(machine_proc_stats.moonraker_stats.len(), 2);
						assert_eq!(machine_proc_stats.moonraker_stats[0].time, 1626612666.850755);
						assert_eq!(machine_proc_stats.moonraker_stats[0].cpu_usage, 2.66);
						assert_eq!(machine_proc_stats.moonraker_stats[0].memory, 24732);
						assert_eq!(machine_proc_stats.moonraker_stats[0].mem_units, "kB");
						assert_eq!(machine_proc_stats.moonraker_stats[1].time, 1626612667.8521338);
						assert_eq!(machine_proc_stats.moonraker_stats[1].cpu_usage, 2.62);
						assert_eq!(machine_proc_stats.moonraker_stats[1].memory, 24732);
						assert_eq!(machine_proc_stats.moonraker_stats[1].mem_units, "kB");
						assert_eq!(machine_proc_stats.throttled_state.bits, 0);
						assert_eq!(machine_proc_stats.throttled_state.flags, Vec::<String>::new());
						assert_eq!(machine_proc_stats.cpu_temp, Some(45.622));
						assert_eq!(machine_proc_stats.network.len(), 2);
						assert_eq!(machine_proc_stats.network["lo"].rx_bytes, 113516429);
						assert_eq!(machine_proc_stats.network["lo"].tx_bytes, 113516429);
						assert_eq!(machine_proc_stats.network["lo"].bandwidth, 3342.68);
						assert_eq!(machine_proc_stats.network["wlan0"].rx_bytes, 48471767);
						assert_eq!(machine_proc_stats.network["wlan0"].tx_bytes, 113430843);
						assert_eq!(machine_proc_stats.network["wlan0"].bandwidth, 4455.91);
						assert_eq!(machine_proc_stats.system_cpu_usage.len(), 5);
						assert_eq!(machine_proc_stats.system_cpu_usage["cpu"], 2.53);
						assert_eq!(machine_proc_stats.system_cpu_usage["cpu0"], 3.03);
						assert_eq!(machine_proc_stats.system_cpu_usage["cpu1"], 5.1);
						assert_eq!(machine_proc_stats.system_cpu_usage["cpu2"], 1.02);
						assert_eq!(machine_proc_stats.system_cpu_usage["cpu3"], 1.0);
						assert_eq!(machine_proc_stats.system_uptime, Some(2876970.38089603));
						assert_eq!(machine_proc_stats.websocket_connections, 4);
					}
					_ => panic!("Expected MachineProcStats"),
				}
			}
			_ => panic!("Expected MoonResult"),
		}
	}

	#[test]
	fn test_serialize_machine_proc_stats() {
		let machine_proc_stats = MachineProcStats {
			moonraker_stats: vec![
				MoonrakerStats {
					time: 1626612666.850755,
					cpu_usage: 2.66,
					memory: 24732,
					mem_units: "kB".to_string(),
				},
				MoonrakerStats {
					time: 1626612667.8521338,
					cpu_usage: 2.62,
					memory: 24732,
					mem_units: "kB".to_string(),
				},
			],
			throttled_state: CpuThrottledState { bits: 0, flags: vec![] },
			cpu_temp: Some(45.622),
			network: BTreeMap::from([
				(
					"lo".to_string(),
					NetworkData {
						rx_bytes: 113516429,
						tx_bytes: 113516429,
						rx_packets: None,
						tx_packets: None,
						rx_errs: None,
						tx_errs: None,
						rx_drop: None,
						tx_drop: None,
						bandwidth: 3342.68,
					},
				),
				(
					"wlan0".to_string(),
					NetworkData {
						rx_bytes: 48471767,
						tx_bytes: 113430843,
						rx_packets: None,
						tx_packets: None,
						rx_errs: None,
						tx_errs: None,
						rx_drop: None,
						tx_drop: None,
						bandwidth: 4455.91,
					},
				),
			]),
			system_cpu_usage: BTreeMap::from([
				("cpu".to_string(), 2.53),
				("cpu0".to_string(), 3.03),
				("cpu1".to_string(), 5.1),
				("cpu2".to_string(), 1.02),
				("cpu3".to_string(), 1.0),
			]),
			system_uptime: Some(2876970.38089603),
			websocket_connections: 4,
		};

		let response = MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::MachineProcStats(machine_proc_stats),
			id: 345,
		};

		let json = serde_json::to_string(&response).unwrap();
		let expected_json = r#"{"jsonrpc":"2.0","result":{"moonraker_stats":[{"time":1626612666.850755,"cpu_usage":2.66,"memory":24732,"mem_units":"kB"},{"time":1626612667.8521338,"cpu_usage":2.62,"memory":24732,"mem_units":"kB"}],"throttled_state":{"bits":0,"flags":[]},"cpu_temp":45.622,"network":{"lo":{"rx_bytes":113516429,"tx_bytes":113516429,"bandwidth":3342.68},"wlan0":{"rx_bytes":48471767,"tx_bytes":113430843,"bandwidth":4455.91}},"system_cpu_usage":{"cpu":2.53,"cpu0":3.03,"cpu1":5.1,"cpu2":1.02,"cpu3":1.0},"system_uptime":2876970.38089603,"websocket_connections":4},"id":345}"#;

		assert_eq!(json, expected_json);
	}
}
