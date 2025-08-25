use moonsock::{MoonNotification, MoonResponse, NotificationMethod, NotificationParam};
use serde_json::json;

#[test]
fn test_parse_notification_gcode_response() {
	let json = json!({
		"jsonrpc": "2.0",
		"method": "notify_gcode_response",
		"params": ["response message"]
	});
	let notification: MoonNotification = serde_json::from_value(json).unwrap();
	assert_eq!(notification.method, NotificationMethod::NotifyGcodeResponse);
	assert_eq!(
		notification.params,
		Some(NotificationParam::String(vec!["response message".to_string()]))
	);

	// match notification {
	// 	MoonNotification { method, params, .. } => {
	// 		assert_eq!(method, NotificationMethod::NotifyGcodeResponse);
	// 		assert_eq!(params, Some(NotificationParam::String(vec!["response message".to_string()])));
	// 	}
	// 	_ => panic!("Invalid response type"),
	// }
}


#[test]
fn notify_gcode_response() {
	let msg_struct = MoonNotification {
		jsonrpc: moonsock::JsonRpcVersion::V2,
		method: moonsock::NotificationMethod::NotifyGcodeResponse,
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

	let msg: MoonNotification = serde_json::from_str(message).unwrap();
	println!("{msg:?}");
	let meg_string = serde_json::to_string(&msg).unwrap();
	println!("{meg_string}");

	match msg {
		MoonNotification { params, .. } => {
			println!("Params: {params:?}");
			match params.unwrap().clone() {
				NotificationParam::String(message) => {
					assert_eq!(
						message,
						vec!["!! Must home axis first: 160.200 210.000 50.022 [7013.719]".to_string()]
					);
				}
				_ => {
					panic!("Wrong message type");
				}
			}
		}
		_ => panic!("Should have gotten a MoonNotification"),
	}
}
