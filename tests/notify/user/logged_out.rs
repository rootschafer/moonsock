use moonsock::{JsonRpcVersion, MoonNotification, NotificationMethod, NotificationParam, UserParam};

#[test]
fn test_deserialize_notify_user_logged_outs() {
	let json = r#"{
        "jsonrpc": "2.0",
        "method": "notify_user_logged_out",
        "params": [
            {
                "username": "testuser"
            }
        ]
    }"#;

	let expected = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyUserLoggedOut,
		params: Some(NotificationParam::User(vec![UserParam { username: "testuser".to_string() }])),
	};

	// let actual: MoonResponse = serde_json::from_str(json).unwrap();
	let actual: MoonNotification = serde_json::from_str(json).unwrap();
	assert_eq!(actual, expected);
}

#[test]
fn test_serialize_notify_user_logged_out() {
	let data = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyUserLoggedOut,
		params: Some(NotificationParam::User(vec![UserParam { username: "testuser".to_string() }])),
	};

	let expected = r#"{"jsonrpc":"2.0","method":"notify_user_logged_out","params":[{"username":"testuser"}]}"#;
	let actual = serde_json::to_string(&data).unwrap();
	assert_eq!(actual, expected);
}
