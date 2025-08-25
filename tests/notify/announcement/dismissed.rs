use moonsock::{EntryId, JsonRpcVersion, MoonNotification, NotificationMethod, NotificationParam};

#[test]
fn test_serialize_notify_announcement_dismissed() {
	let message = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyAnnouncementDismissed,
		params: Some(NotificationParam::AnnouncementEntryId(EntryId {
			entry_id: "arksine/moonlight/issue/3".to_string(),
		})),
	};

	let expected_json = r#"{"jsonrpc":"2.0","method":"notify_announcement_dismissed","params":[{"entry_id":"arksine/moonlight/issue/3"}]}"#;
	let actual_json = serde_json::to_string(&message).unwrap();

	assert_eq!(expected_json, actual_json);
}

#[test]
fn test_deserialize_notify_announcement_dismissed() {
	let json = r#"{"jsonrpc":"2.0","method":"notify_announcement_dismissed","params":[{"entry_id":"arksine/moonlight/issue/3"}]}"#;
	let expected_message = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyAnnouncementDismissed,
		params: Some(NotificationParam::AnnouncementEntryId(EntryId {
			entry_id: "arksine/moonlight/issue/3".to_string(),
		})),
	};

	let actual_message: MoonNotification = serde_json::from_str(json).unwrap();

	assert_eq!(expected_message, actual_message);
}
