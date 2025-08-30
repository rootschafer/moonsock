use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CpuInfo {
	pub cpu_count: u32,
	pub bits: String,
	pub processor: String,
	pub cpu_desc: String,
	pub serial_number: String,
	pub hardware_desc: String,
	pub model: String,
	pub total_memory: u64,
	pub memory_units: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SdInfo {
	pub manufacturer_id: String,
	pub manufacturer: String,
	pub oem_id: String,
	pub product_name: String,
	pub product_revision: String,
	pub serial_number: String,
	pub manufacturer_date: String,
	pub capacity: String,
	pub total_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Distribution {
	pub name: String,
	pub id: String,
	pub version: String,
	pub version_parts: DistributionVersionParts,
	pub like: String,
	pub codename: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DistributionVersionParts {
	pub major: String,
	pub minor: String,
	pub build_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServiceState {
	pub active_state: String,
	pub sub_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Virtualization {
	pub virt_type: String,
	pub virt_identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PythonVersion {
	pub version: Vec<serde_json::Value>,
	pub version_string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkInterface {
	pub mac_address: String,
	pub ip_addresses: Vec<NetworkAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkAddress {
	pub family: String,
	pub address: String,
	pub is_link_local: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CanbusInterface {
	pub tx_queue_len: u32,
	pub bitrate: u32,
	pub driver: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemInfo {
	pub cpu_info: CpuInfo,
	pub sd_info: Option<SdInfo>,
	pub distribution: Distribution,
	pub available_services: Vec<String>,
	pub instance_ids: BTreeMap<String, String>,
	pub service_state: BTreeMap<String, ServiceState>,
	pub virtualization: Virtualization,
	pub python: PythonVersion,
	pub network: BTreeMap<String, NetworkInterface>,
	pub canbus: BTreeMap<String, CanbusInterface>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MachineSystemInfoResponse {
	system_info: SystemInfo,
}


#[cfg(test)]
mod tests {
	use super::*;
	use crate::{response::MoonResultData, JsonRpcVersion, MoonResponse};

	#[test]
	fn test_deserialize_system_info() {
		let system_info = SystemInfo {
			cpu_info: CpuInfo {
				cpu_count: 4,
				bits: "32bit".to_string(),
				processor: "armv7l".to_string(),
				cpu_desc: "ARMv7 Processor rev 4 (v7l)".to_string(),
				serial_number: "b898bdb4".to_string(),
				hardware_desc: "BCM2835".to_string(),
				model: "Raspberry Pi 3 Model B Rev 1.2".to_string(),
				total_memory: 945364,
				memory_units: "kB".to_string(),
			},
			sd_info: Some(SdInfo {
				manufacturer_id: "03".to_string(),
				manufacturer: "Sandisk".to_string(),
				oem_id: "5344".to_string(),
				product_name: "SU32G".to_string(),
				product_revision: "8.0".to_string(),
				serial_number: "46ba46".to_string(),
				manufacturer_date: "4/2018".to_string(),
				capacity: "29.7 GiB".to_string(),
				total_bytes: 31914983424,
			}),
			distribution: Distribution {
				name: "Raspbian GNU/Linux 10 (buster)".to_string(),
				id: "raspbian".to_string(),
				version: "10".to_string(),
				version_parts: DistributionVersionParts {
					major: "10".to_string(),
					minor: "".to_string(),
					build_number: "".to_string(),
				},
				like: "debian".to_string(),
				codename: "buster".to_string(),
			},
			available_services: vec!["klipper".to_string(), "klipper_mcu".to_string(), "moonraker".to_string()],
			instance_ids: {
				let mut map = BTreeMap::new();
				map.insert("moonraker".to_string(), "moonraker".to_string());
				map.insert("klipper".to_string(), "klipper".to_string());
				map
			},
			service_state: {
				let mut map = BTreeMap::new();
				map.insert(
					"klipper".to_string(),
					ServiceState {
						active_state: "active".to_string(),
						sub_state: "running".to_string(),
					},
				);
				map.insert(
					"klipper_mcu".to_string(),
					ServiceState {
						active_state: "active".to_string(),
						sub_state: "running".to_string(),
					},
				);
				map.insert(
					"moonraker".to_string(),
					ServiceState {
						active_state: "active".to_string(),
						sub_state: "running".to_string(),
					},
				);
				map
			},
			virtualization: Virtualization {
				virt_type: "none".to_string(),
				virt_identifier: "none".to_string(),
			},
			python: PythonVersion {
				version: vec![
					serde_json::Value::Number(3.into()),
					serde_json::Value::Number(7.into()),
					serde_json::Value::Number(3.into()),
					serde_json::Value::String("final".to_string()),
					serde_json::Value::Number(0.into()),
				],
				version_string: "3.7.3 (default, Jan 22 2021, 20:04:44)  [GCC 8.3.0]".to_string(),
			},
			network: {
				let mut map = BTreeMap::new();
				map.insert(
					"wlan0".to_string(),
					NetworkInterface {
						mac_address: "00-B0-D0-63-C2-26".to_string(),
						ip_addresses: vec![
							NetworkAddress {
								family: "ipv4".to_string(),
								address: "192.168.1.127".to_string(),
								is_link_local: false,
							},
							NetworkAddress {
								family: "ipv6".to_string(),
								address: "00-B0-D0-63-C2-26".to_string(),
								is_link_local: false,
							},
							NetworkAddress {
								family: "ipv6".to_string(),
								address: "fe80::00-B0-D0-63-C2-26".to_string(),
								is_link_local: true,
							},
						],
					},
				);
				map
			},
			canbus: {
				let mut map = BTreeMap::new();
				map.insert(
					"can0".to_string(),
					CanbusInterface {
						tx_queue_len: 128,
						bitrate: 500000,
						driver: "mcp251x".to_string(),
					},
				);
				map.insert(
					"can1".to_string(),
					CanbusInterface {
						tx_queue_len: 128,
						bitrate: 500000,
						driver: "gs_usb".to_string(),
					},
				);
				map
			},
		};

		let expected = MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::SystemInfo { system_info },
			id: 345,
		};

		let json = r#"{"jsonrpc":"2.0","result":{"system_info":{"cpu_info":{"cpu_count":4,"bits":"32bit","processor":"armv7l","cpu_desc":"ARMv7 Processor rev 4 (v7l)","serial_number":"b898bdb4","hardware_desc":"BCM2835","model":"Raspberry Pi 3 Model B Rev 1.2","total_memory":945364,"memory_units":"kB"},"sd_info":{"manufacturer_id":"03","manufacturer":"Sandisk","oem_id":"5344","product_name":"SU32G","product_revision":"8.0","serial_number":"46ba46","manufacturer_date":"4/2018","capacity":"29.7 GiB","total_bytes":31914983424},"distribution":{"name":"Raspbian GNU/Linux 10 (buster)","id":"raspbian","version":"10","version_parts":{"major":"10","minor":"","build_number":""},"like":"debian","codename":"buster"},"available_services":["klipper","klipper_mcu","moonraker"],"instance_ids":{"moonraker":"moonraker","klipper":"klipper"},"service_state":{"klipper":{"active_state":"active","sub_state":"running"},"klipper_mcu":{"active_state":"active","sub_state":"running"},"moonraker":{"active_state":"active","sub_state":"running"}},"virtualization":{"virt_type":"none","virt_identifier":"none"},"python":{"version":[3,7,3,"final",0],"version_string":"3.7.3 (default, Jan 22 2021, 20:04:44)  [GCC 8.3.0]"},"network":{"wlan0":{"mac_address":"00-B0-D0-63-C2-26","ip_addresses":[{"family":"ipv4","address":"192.168.1.127","is_link_local":false},{"family":"ipv6","address":"00-B0-D0-63-C2-26","is_link_local":false},{"family":"ipv6","address":"fe80::00-B0-D0-63-C2-26","is_link_local":true}]}},"canbus":{"can0":{"tx_queue_len":128,"bitrate":500000,"driver":"mcp251x"},"can1":{"tx_queue_len":128,"bitrate":500000,"driver":"gs_usb"}}}},"id":345}"#;
		let actual: MoonResponse = serde_json::from_str(json).unwrap();
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_serialize_system_info() {
		let system_info = SystemInfo {
			cpu_info: CpuInfo {
				cpu_count: 4,
				bits: "32bit".to_string(),
				processor: "armv7l".to_string(),
				cpu_desc: "ARMv7 Processor rev 4 (v7l)".to_string(),
				serial_number: "b898bdb4".to_string(),
				hardware_desc: "BCM2835".to_string(),
				model: "Raspberry Pi 3 Model B Rev 1.2".to_string(),
				total_memory: 945364,
				memory_units: "kB".to_string(),
			},
			sd_info: Some(SdInfo {
				manufacturer_id: "03".to_string(),
				manufacturer: "Sandisk".to_string(),
				oem_id: "5344".to_string(),
				product_name: "SU32G".to_string(),
				product_revision: "8.0".to_string(),
				serial_number: "46ba46".to_string(),
				manufacturer_date: "4/2018".to_string(),
				capacity: "29.7 GiB".to_string(),
				total_bytes: 31914983424,
			}),
			distribution: Distribution {
				name: "Raspbian GNU/Linux 10 (buster)".to_string(),
				id: "raspbian".to_string(),
				version: "10".to_string(),
				version_parts: DistributionVersionParts {
					major: "10".to_string(),
					minor: "".to_string(),
					build_number: "".to_string(),
				},
				like: "debian".to_string(),
				codename: "buster".to_string(),
			},
			available_services: vec!["klipper".to_string(), "klipper_mcu".to_string(), "moonraker".to_string()],
			instance_ids: {
				let mut map = BTreeMap::new();
				map.insert("moonraker".to_string(), "moonraker".to_string());
				map.insert("klipper".to_string(), "klipper".to_string());
				map
			},
			service_state: {
				let mut map = BTreeMap::new();
				map.insert(
					"klipper".to_string(),
					ServiceState {
						active_state: "active".to_string(),
						sub_state: "running".to_string(),
					},
				);
				map.insert(
					"klipper_mcu".to_string(),
					ServiceState {
						active_state: "active".to_string(),
						sub_state: "running".to_string(),
					},
				);
				map.insert(
					"moonraker".to_string(),
					ServiceState {
						active_state: "active".to_string(),
						sub_state: "running".to_string(),
					},
				);
				map
			},
			virtualization: Virtualization {
				virt_type: "none".to_string(),
				virt_identifier: "none".to_string(),
			},
			python: PythonVersion {
				version: vec![
					serde_json::Value::Number(3.into()),
					serde_json::Value::Number(7.into()),
					serde_json::Value::Number(3.into()),
					serde_json::Value::String("final".to_string()),
					serde_json::Value::Number(0.into()),
				],
				version_string: "3.7.3 (default, Jan 22 2021, 20:04:44)  [GCC 8.3.0]".to_string(),
			},
			network: {
				let mut map = BTreeMap::new();
				map.insert(
					"wlan0".to_string(),
					NetworkInterface {
						mac_address: "00-B0-D0-63-C2-26".to_string(),
						ip_addresses: vec![
							NetworkAddress {
								family: "ipv4".to_string(),
								address: "192.168.1.127".to_string(),
								is_link_local: false,
							},
							NetworkAddress {
								family: "ipv6".to_string(),
								address: "00-B0-D0-63-C2-26".to_string(),
								is_link_local: false,
							},
							NetworkAddress {
								family: "ipv6".to_string(),
								address: "fe80::00-B0-D0-63-C2-26".to_string(),
								is_link_local: true,
							},
						],
					},
				);
				map
			},
			canbus: {
				let mut map = BTreeMap::new();
				map.insert(
					"can0".to_string(),
					CanbusInterface {
						tx_queue_len: 128,
						bitrate: 500000,
						driver: "mcp251x".to_string(),
					},
				);
				map.insert(
					"can1".to_string(),
					CanbusInterface {
						tx_queue_len: 128,
						bitrate: 500000,
						driver: "gs_usb".to_string(),
					},
				);
				map
			},
		};

		let response = MoonResponse::MoonResult {
			jsonrpc: JsonRpcVersion::V2,
			result: MoonResultData::SystemInfo { system_info },
			id: 345,
		};

		let expected = r#"{"jsonrpc":"2.0","result":{"system_info":{"cpu_info":{"cpu_count":4,"bits":"32bit","processor":"armv7l","cpu_desc":"ARMv7 Processor rev 4 (v7l)","serial_number":"b898bdb4","hardware_desc":"BCM2835","model":"Raspberry Pi 3 Model B Rev 1.2","total_memory":945364,"memory_units":"kB"},"sd_info":{"manufacturer_id":"03","manufacturer":"Sandisk","oem_id":"5344","product_name":"SU32G","product_revision":"8.0","serial_number":"46ba46","manufacturer_date":"4/2018","capacity":"29.7 GiB","total_bytes":31914983424},"distribution":{"name":"Raspbian GNU/Linux 10 (buster)","id":"raspbian","version":"10","version_parts":{"major":"10","minor":"","build_number":""},"like":"debian","codename":"buster"},"available_services":["klipper","klipper_mcu","moonraker"],"instance_ids":{"klipper":"klipper","moonraker":"moonraker"},"service_state":{"klipper":{"active_state":"active","sub_state":"running"},"klipper_mcu":{"active_state":"active","sub_state":"running"},"moonraker":{"active_state":"active","sub_state":"running"}},"virtualization":{"virt_type":"none","virt_identifier":"none"},"python":{"version":[3,7,3,"final",0],"version_string":"3.7.3 (default, Jan 22 2021, 20:04:44)  [GCC 8.3.0]"},"network":{"wlan0":{"mac_address":"00-B0-D0-63-C2-26","ip_addresses":[{"family":"ipv4","address":"192.168.1.127","is_link_local":false},{"family":"ipv6","address":"00-B0-D0-63-C2-26","is_link_local":false},{"family":"ipv6","address":"fe80::00-B0-D0-63-C2-26","is_link_local":true}]}},"canbus":{"can0":{"tx_queue_len":128,"bitrate":500000,"driver":"mcp251x"},"can1":{"tx_queue_len":128,"bitrate":500000,"driver":"gs_usb"}}}},"id":345}"#;
		let actual = serde_json::to_string(&response).unwrap();
		println!("Actual:\n{}", serde_json::to_string_pretty(&response).unwrap());
		assert_eq!(actual, expected);
	}
}
