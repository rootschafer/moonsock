use moonsock::{
	response::ServiceState, JsonRpcVersion, MoonNotification, MoonResponse, NotificationMethod, NotificationParam,
	ServiceStateChangedParam,
};

#[test]
fn test_deserialize_notify_service_state_changed() {
	let json = r#"{
        "jsonrpc": "2.0",
        "method": "notify_service_state_changed",
        "params": [
            {
                "klipper": {
                    "active_state": "inactive",
                    "sub_state": "dead"
                }
            }
        ]
    }"#;

	let expected = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyServiceStateChanged,
		params: Some(NotificationParam::ServiceStateChanged(vec![ServiceStateChangedParam {
			services: vec![(
				"klipper".to_string(),
				ServiceState {
					active_state: "inactive".to_string(),
					sub_state: "dead".to_string(),
				},
			)]
			.into_iter()
			.collect(),
		}])),
	};

	// let actual: MoonResponse = serde_json::from_str(json).unwrap();
	let actual: MoonNotification = serde_json::from_str(json).unwrap();
	assert_eq!(actual, expected);
}

#[test]
fn test_serialize_notify_service_state_changed() {
	let data = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyServiceStateChanged,
		params: Some(NotificationParam::ServiceStateChanged(vec![ServiceStateChangedParam {
			services: vec![(
				"klipper".to_string(),
				ServiceState {
					active_state: "inactive".to_string(),
					sub_state: "dead".to_string(),
				},
			)]
			.into_iter()
			.collect(),
		}])),
	};

	let expected = r#"{"jsonrpc":"2.0","method":"notify_service_state_changed","params":[{"klipper":{"active_state":"inactive","sub_state":"dead"}}]}"#;
	let actual = serde_json::to_string(&data).unwrap();
	assert_eq!(actual, expected);
}

