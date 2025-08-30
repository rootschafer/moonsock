use moonsock::{
	JobQueueAction, JobQueueChangedParam, JsonRpcVersion, MoonNotification, NotificationMethod, NotificationParam,
};

// #[test]
// fn test_deserialize_notify_job_queue_changed() {
//     let json = r#"{
//         "jsonrpc": "2.0",
//         "method": "notify_job_queue_changed",
//         "params": [
//             {
//                 "action": "state_changed",
//                 "updated_queue": null,
//                 "queue_state": "paused"
//             }
//         ]
//     }"#;

//     let expected = MoonNotification {
//         jsonrpc: JsonRpcVersion::V2,
//         method: NotificationMethod::NotifyJobQueueChanged,
//         params: Some(NotificationParam::JobQueueChanged(vec![
//             JobQueueChangedParam {
//                 action: "state_changed".to_string(),
//                 updated_queue: None,
//                 queue_state: "paused".to_string(),
//             },
//         ])),
//     };

//     let actual: MoonResponse = serde_json::from_str(json).unwrap();
//     assert_eq!(actual, expected);
// }

// #[test]
// fn test_serialize_notify_job_queue_changed() {
//     let data = MoonNotification {
//         jsonrpc: JsonRpcVersion::V2,
//         method: NotificationMethod::NotifyJobQueueChanged,
//         params: Some(NotificationParam::JobQueueChanged(vec![
//             JobQueueChangedParam {
//                 action: "state_changed".to_string(),
//                 updated_queue: None,
//                 queue_state: "paused".to_string(),
//             },
//         ])),
//     };

//     let expected = r#"{"jsonrpc":"2.0","method":"notify_job_queue_changed","params":[{"action":"state_changed","updated_queue":null,"queue_state":"paused"}]}"#;
//     let actual = serde_json::to_string(&data).unwrap();
//     assert_eq!(actual, expected);
// }


#[test]
fn test_deserialize_notify_job_queue_changed() {
	let json = r#"{
        "jsonrpc": "2.0",
        "method": "notify_job_queue_changed",
        "params": [
            {
                "action": "state_changed",
                "updated_queue": null,
                "queue_state": "paused"
            }
        ]
    }"#;

	let expected = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyJobQueueChanged,
		params: Some(NotificationParam::JobQueueChanged(vec![JobQueueChangedParam {
			action: JobQueueAction::StateChanged,
			updated_queue: None,
			queue_state: "paused".to_string(),
		}])),
	};

	// let actual: MoonResponse = serde_json::from_str(json).unwrap();
	let actual: MoonNotification = serde_json::from_str(json).unwrap();
	assert_eq!(actual, expected);
}

#[test]
fn test_serialize_notify_job_queue_changed() {
	let data = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyJobQueueChanged,
		params: Some(NotificationParam::JobQueueChanged(vec![JobQueueChangedParam {
			action: JobQueueAction::StateChanged,
			updated_queue: None,
			queue_state: "paused".to_string(),
		}])),
	};

	let expected = r#"{"jsonrpc":"2.0","method":"notify_job_queue_changed","params":[{"action":"state_changed","updated_queue":null,"queue_state":"paused"}]}"#;
	let actual = serde_json::to_string(&data).unwrap();
	assert_eq!(actual, expected);
}
