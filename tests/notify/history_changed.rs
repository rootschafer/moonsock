use moonsock::{
	HistoryChangedParam, JobObject, JsonRpcVersion, MoonNotification, NotificationMethod, NotificationParam,
};

#[test]
fn test_deserialize_notify_history_changed() {
	let json = r#"{
        "jsonrpc": "2.0",
        "method": "notify_history_changed",
        "params": [
            {
                "action": "added",
                "job": {
                    "job_id": "000001",
                    "exists": true,
                    "end_time": 1615764265.6493807,
                    "filament_used": 7.83,
                    "filename": "test/history_test.gcode",
                    "metadata": {},
                    "print_duration": 18.37201827496756,
                    "status": "completed",
                    "start_time": 1615764496.622146,
                    "total_duration": 18.37201827496756
                }
            }
        ]
    }"#;

	let expected = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyHistoryChanged,
		params: Some(NotificationParam::HistoryChanged(vec![HistoryChangedParam {
			action: "added".to_string(),
			job: JobObject {
				job_id: "000001".to_string(),
				exists: true,
				end_time: 1615764265.6493807,
				filament_used: 7.83,
				filename: "test/history_test.gcode".to_string(),
				metadata: serde_json::json!({}),
				print_duration: 18.37201827496756,
				status: "completed".to_string(),
				start_time: 1615764496.622146,
				total_duration: 18.37201827496756,
			},
		}])),
	};

	let actual: MoonNotification = serde_json::from_str(json).unwrap();
	assert_eq!(actual, expected);
}

#[test]
fn test_serialize_notify_history_changed() {
	let data = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyHistoryChanged,
		params: Some(NotificationParam::HistoryChanged(vec![HistoryChangedParam {
			action: "added".to_string(),
			job: JobObject {
				job_id: "000001".to_string(),
				exists: true,
				end_time: 1615764265.6493807,
				filament_used: 7.83,
				filename: "test/history_test.gcode".to_string(),
				metadata: serde_json::json!({}),
				print_duration: 18.37201827496756,
				status: "completed".to_string(),
				start_time: 1615764496.622146,
				total_duration: 18.37201827496756,
			},
		}])),
	};

	let expected = r#"{"jsonrpc":"2.0","method":"notify_history_changed","params":[{"action":"added","job":{"job_id":"000001","exists":true,"end_time":1615764265.6493807,"filament_used":7.83,"filename":"test/history_test.gcode","metadata":{},"print_duration":18.37201827496756,"status":"completed","start_time":1615764496.622146,"total_duration":18.37201827496756}}]}"#;
	let actual = serde_json::to_string(&data).unwrap();
	assert_eq!(actual, expected);
}
