use moonsock::{
	JsonRpcVersion, MoonNotification, NotificationMethod, NotificationParam, SudoAlertParams,
};

#[test]
fn test_serialize_notify_sudo_alert() {
	let message = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifySudoAlert,
		params: Some(NotificationParam::SudoAlert(SudoAlertParams {
			sudo_requested: true,
			sudo_messages: vec!["Sudo password required to update Moonraker's systemd service.".to_string()],
		})),
	};

	let expected_json = r#"{"jsonrpc":"2.0","method":"notify_sudo_alert","params":[{"sudo_requested":true,"sudo_messages":["Sudo password required to update Moonraker's systemd service."]}]}"#;
	let actual_json = serde_json::to_string(&message).unwrap();

	assert_eq!(expected_json, actual_json);
}

#[test]
fn test_deserialize_notify_sudo_alert() {
	let json = r#"{"jsonrpc":"2.0","method":"notify_sudo_alert","params":[{"sudo_requested":true,"sudo_messages":["Sudo password required to update Moonraker's systemd service."]}]}"#;
	let expected_message = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifySudoAlert,
		params: Some(NotificationParam::SudoAlert(SudoAlertParams {
			sudo_requested: true,
			sudo_messages: vec!["Sudo password required to update Moonraker's systemd service.".to_string()],
		})),
	};

	// let actual_message: MoonResponse = serde_json::from_str(json).unwrap();
	let actual: MoonNotification = serde_json::from_str(json).unwrap();

	assert_eq!(expected_message, actual);
}
