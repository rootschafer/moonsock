use moonsock::{
	MoonMethod,
	// MoonParam
	// MoonMSG,
	MoonRequest,
};

#[test]
fn test_server_info() {
	let json = r#"{
        "jsonrpc": "2.0",
        "method": "server.info",
        "id": 9546
    }"#;

	// let moon_msg: MoonMSG = serde_json::from_str(json).unwrap();
	let moon_msg: MoonRequest = serde_json::from_str(json).unwrap();
	// match moon_msg.method().unwrap() {
	match moon_msg.method {
		MoonMethod::ServerInfo => {}
		_ => panic!("Expected MoonMethod::ServerInfo"),
	}
}

#[test]
fn test_server_config() {
	let json = r#"{
        "jsonrpc": "2.0",
        "method": "server.config",
        "id": 5616
    }"#;

	// let moon_msg: MoonMSG = serde_json::from_str(json).unwrap();
	let moon_msg: MoonRequest = serde_json::from_str(json).unwrap();
	// match moon_msg.method().unwrap() {
	match moon_msg.method {
		MoonMethod::ServerConfig => {}
		_ => panic!("Expected MoonMethod::ServerConfig"),
	}
}

#[test]
fn test_server_temperature_store() {
	let json = r#"{
        "jsonrpc": "2.0",
        "method": "server.temperature_store",
        "params": {
            "include_monitors": false
        },
        "id": 2313
    }"#;

	// let moon_msg: MoonMSG = serde_json::from_str(json).unwrap();
	let moon_msg: MoonRequest = serde_json::from_str(json).unwrap();
	// match moon_msg.method().unwrap() {
	match moon_msg.method {
		MoonMethod::ServerTemperatureStore => {}
		_ => panic!("Expected MoonMethod::ServerTemperatureStore"),
	}
}


#[test]
fn test_serialize_server_info() {
	// let moon_msg = MoonMSG::new(
	//     MoonMethod::ServerInfo,
	//     None,
	//     Some(9546),
	// );
	// let moon_msg = MoonRequest::new(
	//     MoonMethod::ServerInfo,
	//     None,
	// );
	let moon_msg = MoonRequest {
		jsonrpc: moonsock::JsonRpcVersion::V2,
		method: MoonMethod::ServerInfo,
		params: None,
		id: 9546,
	};

	let json = serde_json::to_string(&moon_msg).unwrap();
	assert_eq!(json, r#"{"jsonrpc":"2.0","method":"server.info","id":9546}"#);
}

// #[test]
// fn test_deserialize_server_info() {
//     let json = r#"{"jsonrpc":"2.0","method":"server.info","id":9546}"#;
//     let moon_msg: MoonMSG = serde_json::from_str(json).unwrap();
//     // assert_eq!(moon_msg.jsonrpc, "2.0");
//     assert_eq!(moon_msg.method().unwrap(), MoonMethod::ServerInfo);
//     // assert_eq!(moon_msg.params().unwrap(), None);
//     assert_eq!(moon_msg.id(), Some(9546));
// }

// #[test]
// fn test_serialize_server_config() {
//     let moon_msg = MoonMSG::new(
//         // "2.0",
//         MoonMethod::ServerConfig,
//         None,
//         Some(5616),
//     );
//     let json = serde_json::to_string(&moon_msg).unwrap();
//     assert_eq!(
//         json,
//         r#"{"jsonrpc":"2.0","method":"server.config","id":5616}"#
//     );
// }

// #[test]
// fn test_deserialize_server_config() {
//     let json = r#"{"jsonrpc":"2.0","method":"server.config","id":5616}"#;
//     let moon_msg: MoonMSG = serde_json::from_str(json).unwrap();
//     assert_eq!(moon_msg.jsonrpc, "2.0");
//     assert_eq!(moon_msg.method, MoonMethod::ServerConfig);
//     assert_eq!(moon_msg.params, None);
//     assert_eq!(moon_msg.id, 5616);
// }

// #[test]
// fn test_serialize_server_temperature_store() {
//     let moon_msg = MoonMSG::new(
//         // "2.0",
//         MoonMethod::ServerTemperatureStore,
//         Some(MoonParam::IncludeMonitors(false)),
//         2313,
//     );
//     let json = serde_json::to_string(&moon_msg).unwrap();
//     assert_eq!(
//         json,
//         r#"{"jsonrpc":"2.0","method":"server.temperature_store","params":{"include_monitors":false},"id":2313}"#
//     );
// }

// #[test]
// fn test_deserialize_server_temperature_store() {
//     let json = r#"{"jsonrpc":"2.0","method":"server.temperature_store","params":{"include_monitors":false},"id":2313}"#;
//     let moon_msg: MoonMSG = serde_json::from_str(json).unwrap();
//     assert_eq!(moon_msg.jsonrpc, "2.0");
//     assert_eq!(moon_msg.method, MoonMethod::ServerTemperatureStore);
//     match moon_msg.params {
//         Some(MoonParam::IncludeMonitors(include_monitors)) => {
//             assert!(!include_monitors);
//         }
//         _ => panic!("Expected MoonParam::IncludeMonitors"),
//     }
//     assert_eq!(moon_msg.id, 2313);
// }
