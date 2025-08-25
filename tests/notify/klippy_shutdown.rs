use moonsock::{JsonRpcVersion, MoonNotification, NotificationMethod};

#[test]
fn test_deserialize_notify_klippy_shutdown() {
	let json = r#"{
        "jsonrpc": "2.0",
        "method": "notify_klippy_shutdown"
    }"#;

	let expected = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyKlippyShutdown,
		params: None,
	};

	// let actual: MoonResponse = serde_json::from_str(json).unwrap();
	let actual: MoonNotification = serde_json::from_str(json).unwrap();
	assert_eq!(actual, expected);
}

#[test]
fn test_serialize_notify_klippy_shutdown() {
	let data = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyKlippyShutdown,
		params: None,
	};

	let expected = r#"{"jsonrpc":"2.0","method":"notify_klippy_shutdown"}"#;
	let actual = serde_json::to_string(&data).unwrap();
	assert_eq!(actual, expected);
}
