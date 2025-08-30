use serde_json::json;
use moonsock::{
	jsonrpc_ws_client::JsonRpcVersion, JsonRpcNotification, MoonNotification, NotificationMethod, NotificationParam,
};

#[test]
fn test_moon_notification_try_from_notify_gcode_response() {
	// Test the conversion for gcode response notification
	let json_rpc_notification = JsonRpcNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: "notify_gcode_response".to_string(),
		params: Some(json!(["!! Must home axis first: 160.200 210.000 50.022 [7013.719]"])),
	};

	let moon_notification = MoonNotification::try_from(json_rpc_notification).unwrap();

	assert_eq!(moon_notification.method, NotificationMethod::NotifyGcodeResponse);

	match moon_notification.params {
		Some(NotificationParam::String(messages)) => {
			assert_eq!(messages.len(), 1);
			assert_eq!(messages[0], "!! Must home axis first: 160.200 210.000 50.022 [7013.719]");
		}
		_ => panic!("Expected String parameters"),
	}
}

#[test]
fn test_moon_notification_try_from_notify_klippy_ready() {
	// Test the conversion for klippy ready notification (no params)
	let json_rpc_notification = JsonRpcNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: "notify_klippy_ready".to_string(),
		params: None,
	};

	let moon_notification = MoonNotification::try_from(json_rpc_notification).unwrap();

	assert_eq!(moon_notification.method, NotificationMethod::NotifyKlippyReady);
	assert!(moon_notification.params.is_none());
}

#[test]
fn test_moon_notification_try_from_unknown_method() {
	// Test the conversion with an unknown method
	let json_rpc_notification = JsonRpcNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: "notify_unknown_method".to_string(),
		params: Some(json!(["test"])),
	};

	let result = MoonNotification::try_from(json_rpc_notification);
	assert!(result.is_err());
	assert!(result
		.unwrap_err()
		.to_string()
		.contains("Error parsing notification method"));
}
