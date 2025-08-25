use moonsock::{
	JsonRpcVersion, MoonNotification, NotificationMethod, NotificationParam, UpdateResponseParam,
};

#[test]
fn test_parse_notify_update_response() {
	let json = r#"{
        "jsonrpc": "2.0",
        "method": "notify_update_response",
        "params": [
            {
                "application": "some_app",
                "proc_id": 446461,
                "message": "Update Response Message",
                "complete": false
            }
        ]
    }"#;

	let expected = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyUpdateResponse,
		params: Some(NotificationParam::UpdateResponse(vec![UpdateResponseParam {
			application: "some_app".to_string(),
			proc_id: 446461,
			message: "Update Response Message".to_string(),
			complete: false,
		}])),
	};

	// let actual: MoonResponse = serde_json::from_str(json).unwrap();
	let actual: MoonNotification = serde_json::from_str(json).unwrap();
	assert_eq!(actual, expected);

	let serialized = serde_json::to_string(&actual).unwrap();
	let deserialized: MoonNotification = serde_json::from_str(&serialized).unwrap();
	assert_eq!(deserialized, expected);
}

