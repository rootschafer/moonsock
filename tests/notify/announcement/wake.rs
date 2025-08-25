use moonsock::{EntryId, JsonRpcVersion, MoonNotification, MoonResponse, NotificationMethod, NotificationParam};

#[test]
fn test_serialize_notify_announcement_wake() {
	let message = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyAnnouncementWake,
		params: Some(NotificationParam::AnnouncementEntryId(EntryId {
			entry_id: "arksine/moonlight/issue/1".to_string(),
		})),
	};

	let expected_json =
		r#"{"jsonrpc":"2.0","method":"notify_announcement_wake","params":[{"entry_id":"arksine/moonlight/issue/1"}]}"#;
	let actual_json = serde_json::to_string(&message).unwrap();

	assert_eq!(expected_json, actual_json);
}

// #[test]
// fn test_deserialize_notify_announcement_wake() {
//     let json = r#"{"jsonrpc":"2.0","method":"notify_announcement_wake","params":[{"entry_id":"arksine/moonlight/issue/1"}]}"#;
//     let expected_message = MoonNotification {
//         jsonrpc: JsonRpcVersion::V2,
//         method: NotificationMethod::NotifyAnnouncementWake,
//         params: Some(NotificationParam::AnnouncementWake(AnnouncementDismissedParam {
//             entry_id: "arksine/moonlight/issue/1".to_string(),
//         })),
//     };

//     let actual_message: MoonResponse = serde_json::from_str(json).unwrap();

//     assert_eq!(expected_message, actual_message);
// }

#[test]
fn test_deserialize_notify_announcement_wake() {
	let json =
		r#"{"jsonrpc":"2.0","method":"notify_announcement_wake","params":[{"entry_id":"arksine/moonlight/issue/1"}]}"#;
	let expected_message = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyAnnouncementWake,
		params: Some(NotificationParam::AnnouncementEntryId(EntryId {
			entry_id: "arksine/moonlight/issue/1".to_string(),
		})),
	};

	let actual_message: MoonNotification = serde_json::from_str(json).unwrap();

	assert_eq!(expected_message, actual_message);
}
