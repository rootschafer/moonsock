// use moonsock::{MoonResponse, JsonRpcVersion, response::{MoonResultData, ServerInfo, PrinterState}};

// #[test]
// fn test_deserialize_server_info() {
// let json = r#"{
//     "jsonrpc": "2.0",
//     "result": {
//         "klippy_connected": true,
//         "klippy_state": "ready",
//         "components": ["database", "file_manager"],
//         "failed_components": [],
//         "registered_directories": ["config", "gcodes"],
//         "warnings": ["warning1", "warning2"],
//         "websocket_count": 2,
//         "moonraker_version": "v0.7.1-105-ge4f103c",
//         "api_version": [1, 0, 0],
//         "api_version_string": "1.0.0"
//     },
//     "id": 354
// }"#;

//     let response: MoonResponse = serde_json::from_str(json).unwrap();

//     match response {
//         MoonResponse::MoonResult { result, .. } => match result {
//             MoonResultData::ServerInfo(_) => (),
//             _ => panic!("Expected ServerInfo"),
//         },
//         _ => panic!("Expected MoonResult"),
//     }
// }

// #[test]
// fn test_serialize_server_info() {
//     let server_info = ServerInfo {
//         klippy_connected: true,
//         klippy_state: PrinterState::Ready,
//         components: vec!["database".to_string(), "file_manager".to_string()],
//         failed_components: vec![],
//         registered_directories: vec!["config".to_string(), "gcodes".to_string()],
//         warnings: vec!["warning1".to_string(), "warning2".to_string()],
//         websocket_count: 2,
//         moonraker_version: "v0.7.1-105-ge4f103c".to_string(),
//         api_version: vec![1, 0, 0],
//         api_version_string: "1.0.0".to_string(),
//     };

//     // let response = MoonResponse::MoonResult {
//     //     // jsonrpc: JsonRpcVersion::V2,
//     //     result: MoonResultData::ServerInfo(server_info),
//     //     id: 354,
//     //     ..Default::default()
//     // };
//     let mut response = MoonResponse::default_server_info_result(server_info);
//     response.set_id(354);

//     let json = serde_json::to_string(&response).unwrap();

//     assert!(json.contains(r#""klippy_connected":true"#));
//     assert!(json.contains(r#""klippy_state":"ready""#));
//     assert!(json.contains(r#""components":["database","file_manager"]"#));
//     assert!(json.contains(r#""failed_components":[]"#));
//     assert!(json.contains(r#""registered_directories":["config","gcodes"]"#));
//     assert!(json.contains(r#""warnings":["warning1","warning2"]"#));
//     assert!(json.contains(r#""websocket_count":2"#));
//     assert!(json.contains(r#""moonraker_version":"v0.7.1-105-ge4f103c""#));
//     assert!(json.contains(r#""api_version":[1,0,0]"#));
//     assert!(json.contains(r#""api_version_string":"1.0.0""#));
//     assert!(json.contains(r#""id":354"#));
//     assert!(json.contains(r#""jsonrpc":"2.0""#));
// }

// #[test]
// fn test_serialize_server_info() {
//     let server_info = ServerInfo {
//         klippy_connected: true,
//         klippy_state: PrinterState::Ready,
//         components: vec!["database".to_string(), "file_manager".to_string()],
//         failed_components: vec![],
//         registered_directories: vec!["config".to_string(), "gcodes".to_string()],
//         warnings: vec!["warning1".to_string(), "warning2".to_string()],
//         websocket_count: 2,
//         moonraker_version: "v0.7.1-105-ge4f103c".to_string(),
//         api_version: vec![1, 0, 0],
//         api_version_string: "1.0.0".to_string(),
//     };

//     let response = MoonResponse::MoonResult {
//         jsonrpc: JsonRpcVersion::V2,
//         result: MoonResultData::ServerInfo(server_info),
//         id: 354,
//     };

//     let json = serde_json::to_string(&response).unwrap();

//     assert!(json.contains(r#""klippy_connected":true"#));
//     assert!(json.contains(r#""klippy_state":"ready""#));
//     // ... check other fields ...
// }
