use serde_json::json;
use moonsock::{MoonResponse, NotificationMethod, NotificationParam};

#[test]
fn test_parse_notification_gcode_response() {
	let json = json!({
		"jsonrpc": "2.0",
		"method": "notify_gcode_response",
		"params": ["response message"]
	});
	let notification: MoonResponse = serde_json::from_value(json).unwrap();
	match notification {
		MoonResponse::Notification { method, params, .. } => {
			assert_eq!(method, NotificationMethod::NotifyGcodeResponse);
			// assert_eq!(params, Some(vec![NotificationParam::String("response message".to_string())]));
			assert_eq!(params, Some(NotificationParam::String(vec!["response message".to_string()])));
		}
		_ => panic!("Invalid response type"),
	}
}


#[test]
fn notify_gcode_response() {
	// let msg_struct = MoonMSG::MethodParamVec {
	//     jsonrpc: moonsock::JsonRpcVersion::V2,
	//     method: MoonMethod::NotifyGcodeResponse,
	//     params: vec![MoonParam::NotifyGcodeResponse("!! Must home axis first: 160.200 210.000 50.022 [7013.719]".to_string())],
	// };
	// let msg_struct = MoonMSG::MethodParam {
	let msg_struct = MoonResponse::Notification {
		jsonrpc: moonsock::JsonRpcVersion::V2,
		method: moonsock::NotificationMethod::NotifyGcodeResponse,
		// params: vec![MoonParam::NotifyGcodeResponse("!! Must home axis first: 160.200 210.000 50.022 [7013.719]".to_string())],
		// params: Some(vec![NotificationParam::String("!! Must home axis first: 160.200 210.000 50.022 [7013.719]".to_string())]),
		params: Some(NotificationParam::String(vec![
			"!! Must home axis first: 160.200 210.000 50.022 [7013.719]".to_string(),
		])),
	};
	let msg_struct_string = serde_json::to_string(&msg_struct).unwrap();
	println!("{msg_struct_string}");
	let message = r#"{
        "jsonrpc": "2.0", 
        "method": "notify_gcode_response", 
        "params": [
            "!! Must home axis first: 160.200 210.000 50.022 [7013.719]"
        ]
    }"#;
	// let msg: MoonMSG = serde_json::from_str(message).unwrap();
	let msg: MoonResponse = serde_json::from_str(message).unwrap();
	println!("{msg:?}");
	let meg_string = serde_json::to_string(&msg).unwrap();
	println!("{meg_string}");
	match msg {
		MoonResponse::Notification { params, .. } => {
			println!("Params: {params:?}");
			// match params.unwrap()[0].clone() {
			match params.unwrap().clone() {
				NotificationParam::String(message) => {
					assert_eq!(
						message,
						vec!["!! Must home axis first: 160.200 210.000 50.022 [7013.719]".to_string()]
					);
				}
				// MoonParam::NotifyGcodeResponse(response) => {
				//     assert_eq!(response.clone(), vec!["!! Must home axis first: 160.200 210.000 50.022 [7013.719]"]);
				// }
				_ => {
					panic!("Wrong message type");
				}
			}
		}
		_ => panic!("Should have gotten a MoonResponse::Notification"),
	}
	// match msg.params().unwrap().clone() {
	//     MoonParam::NotifyGcodeResponse(response) => {
	//         assert_eq!(response.clone(), vec!["!! Must home axis first: 160.200 210.000 50.022 [7013.719]"]);
	//     }
	//     _ => panic!("Wrong message type"),
	// }
}

