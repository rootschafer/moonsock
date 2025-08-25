use moonsock::{ActiveSpoolSetParams, JsonRpcVersion, MoonResponse, NotificationMethod, NotificationParam};

#[test]
fn test_serialize_notify_active_spool_set() {
	let message = MoonResponse::Notification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyActiveSpoolSet,
		params: Some(NotificationParam::ActiveSpoolSet(ActiveSpoolSetParams { spool_id: 1 })),
	};

	let expected_json = r#"{"jsonrpc":"2.0","method":"notify_active_spool_set","params":[{"spool_id":1}]}"#;
	let actual_json = serde_json::to_string(&message).unwrap();

	assert_eq!(expected_json, actual_json);
}

#[test]
fn test_deserialize_notify_active_spool_set() {
	let json = r#"{"jsonrpc":"2.0","method":"notify_active_spool_set","params":[{"spool_id":1}]}"#;
	let expected_message = MoonResponse::Notification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyActiveSpoolSet,
		params: Some(NotificationParam::ActiveSpoolSet(ActiveSpoolSetParams { spool_id: 1 })),
	};

	let actual_message: MoonResponse = serde_json::from_str(json).unwrap();

	assert_eq!(expected_message, actual_message);
}

