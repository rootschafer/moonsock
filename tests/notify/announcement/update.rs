use moonsock::{
	AnnouncementEntry, AnnouncementUpdateParam, JsonRpcVersion, MoonNotification, NotificationMethod,
	NotificationParam,
};

// #[test]
// fn test_serialize_announcement() {
//     let notification = MoonNotification {
//         jsonrpc: JsonRpcVersion::V2,
//         method: NotificationMethod::NotifyAnnouncementUpdate,
//         params: Some(NotificationParam::Announcement(AnnouncementParams {
//             entries: vec![
//                 AnnouncementEntry {
//                     entry_id: "arksine/moonlight/issue/3".to_string(),
//                     url: "https://github.com/Arksine/moonlight/issues/3".to_string(),
//                     title: "Test announcement 3".to_string(),
//                     description: "Test Description [with a link](https://moonraker.readthedocs.io).".to_string(),
//                     priority: "normal".to_string(),
//                     date: 1647459219,
//                     dismissed: false,
//                     date_dismissed: None,
//                     dismiss_wake: None,
//                     source: "moonlight".to_string(),
//                     feed: "moonlight".to_string(),
//                 },
//                 AnnouncementEntry {
//                     entry_id: "arksine/moonlight/issue/2".to_string(),
//                     url: "https://github.com/Arksine/moonlight/issues/2".to_string(),
//                     title: "Announcement Test Two".to_string(),
//                     description: "This is a high priority announcement. This line is included in the description.".to_string(),
//                     priority: "high".to_string(),
//                     date: 1646855579,
//                     dismissed: false,
//                     date_dismissed: None,
//                     dismiss_wake: None,
//                     source: "moonlight".to_string(),
//                     feed: "moonlight".to_string(),
//                 },
//                 AnnouncementEntry {
//                     entry_id: "arksine/moonraker/issue/349".to_string(),
//                     url: "https://github.com/Arksine/moonraker/issues/349".to_string(),
//                     title: "PolicyKit warnings; unable to manage services, restart system, or update packages".to_string(),
//                     description: "This announcement is an effort to get ahead of a coming change that will certainly result in issues.  PR #346  has been merged, and with it are some changes to Moonraker's default behavior.".to_string(),
//                     priority: "normal".to_string(),
//                     date: 1643392406,
//                     dismissed: false,
//                     date_dismissed: None,
//                     dismiss_wake: None,
//                     source: "moonlight".to_string(),
//                     feed: "Moonraker".to_string(),
//                 }
//             ],
//         })),
//     };

//     let serialized = serde_json::to_string(&notification).unwrap();
//     assert_eq!(
//         serialized,
//         // r#"{"jsonrpc":"2.0","method":"notify_announcement_update","params":[{"entries":[{"entry_id":"arksine/moonlight/issue/3","url":"https://github.com/Arksine/moonlight/issues/3","title":"Test announcement 3","description":"Test Description [with a link](https://moonraker.readthedocs.io).","priority":"normal","date":1647459219,"dismissed":false,"date_dismissed":null,"dismiss_wake":null,"source":"moonlight","feed":"moonlight"},/*...*/]}]}"#
//         // r#"{"jsonrpc":"2.0","method":"notify_announcement_update","params":[{"entries":[{"entry_id":"arksine/moonlight/issue/3","url":"https://github.com/Arksine/moonlight/issues/3","title":"Test announcement 3","description":"Test Description [with a link](https://moonraker.readthedocs.io).","priority":"normal","date":1647459219,"dismissed":false,"source":"moonlight","feed":"moonlight"}]}]}"#
//         r##"{"jsonrpc":"2.0","method":"notify_announcement_update","params":[{"entries":[{"entry_id":"arksine/moonlight/issue/3","url":"https://github.com/Arksine/moonlight/issues/3","title":"Test announcement 3","description":"Test Description [with a link](https://moonraker.readthedocs.io).","priority":"normal","date":1647459219,"dismissed":false,"date_dismissed":null,"dismiss_wake":null,"source":"moonlight","feed":"moonlight"},{"entry_id":"arksine/moonlight/issue/2","url":"https://github.com/Arksine/moonlight/issues/2","title":"Announcement Test Two","description":"This is a high priority announcement. This line is included in the description.","priority":"high","date":1646855579,"dismissed":false,"date_dismissed":null,"dismiss_wake":null,"source":"moonlight","feed":"moonlight"},{"entry_id":"arksine/moonraker/issue/349","url":"https://github.com/Arksine/moonraker/issues/349","title":"PolicyKit warnings; unable to manage services, restart system, or update packages","description":"This announcement is an effort to get ahead of a coming change that will certainly result in issues.  PR #346  has been merged, and with it are some changes to Moonraker's default behavior.","priority":"normal","date":1643392406,"dismissed":false,"source":"moonlight","feed":"Moonraker"}]}]}"##
//     );
// }

// #[test]
// fn test_deserialize_announcement() {
//     // let json = r#"{"jsonrpc":"2.0","method":"notify_announcement_update","params":[{"entries":[{"entry_id":"arksine/moonlight/issue/3","url":"https://github.com/Arksine/moonlight/issues/3","title":"Test announcement 3","description":"Test Description [with a link](https://moonraker.readthedocs.io).","priority":"normal","date":1647459219,"dismissed":false,"date_dismissed":null,"dismiss_wake":null,"source":"moonlight","feed":"moonlight"},/*...*/]}]}"#;
//     // let json = r#"{"jsonrpc":"2.0","method":"notify_announcement_update","params":[{"entries":[{"entry_id":"arksine/moonlight/issue/3","url":"https://github.com/Arksine/moonlight/issues/3","title":"Test announcement 3","description":"Test Description [with a link](https://moonraker.readthedocs.io).","priority":"normal","date":1647459219,"dismissed":false,"source":"moonlight","feed":"moonlight"}]}]}"#;
//     // let json = r##"{"jsonrpc":"2.0","method":"notify_announcement_update","params":[{"entries":[{"entry_id":"arksine/moonlight/issue/3","url":"https://github.com/Arksine/moonlight/issues/3","title":"Test announcement 3","description":"Test Description [with a link](https://moonraker.readthedocs.io).","priority":"normal","date":1647459219,"dismissed":false,"date_dismissed":null,"dismiss_wake":null,"source":"moonlight","feed":"moonlight"},{"entry_id":"arksine/moonlight/issue/2","url":"https://github.com/Arksine/moonlight/issues/2","title":"Announcement Test Two","description":"This is a high priority announcement. This line is included in the description.","priority":"high","date":1646855579,"dismissed":false,"date_dismissed":null,"dismiss_wake":null,"source":"moonlight","feed":"moonlight"},{"entry_id":"arksine/moonraker/issue/349","url":"https://github.com/Arksine/moonraker/issues/349","title":"PolicyKit warnings; unable to manage services, restart system, or update packages","description":"This announcement is an effort to get ahead of a coming change that will certainly result in issues.  PR #346  has been merged, and with it are some changes to Moonraker's default behavior.","priority":"normal","date":1643392406,"dismissed":false,"source":"moonlight","feed":"Moonraker"}]}]}"##;
//     let json = r#"{"jsonrpc":"2.0","method":"notify_announcement_update","params":[{"entries":[{"entry_id":"arksine/moonlight/issue/3","url":"https://github.com/Arksine/moonlight/issues/3","title":"Test announcement 3","description":"Test Description [with a link](https://moonraker.readthedocs.io).","priority":"normal","date":1647459219,"dismissed":false,"date_dismissed":null,"dismiss_wake":null,"source":"moonlight","feed":"moonlight"},{"entry_id":"arksine/moonlight/issue/2","url":"https://github.com/Arksine/moonlight/issues/2","title":"Announcement Test Two","description":"This is a high priority announcement. This line is included in the description.","priority":"high","date":1646855579,"dismissed":false,"date_dismissed":null,"dismiss_wake":null,"source":"moonlight","feed":"moonlight"},{"entry_id":"arksine/moonraker/issue/349","url":"https://github.com/Arksine/moonraker/issues/349","title":"PolicyKit warnings; unable to manage services, restart system, or update packages","description":"This announcement is an effort to get ahead of a coming change that will certainly result in issues.  PR #346  has been merged, and with it are some changes to Moonraker's default behavior.","priority":"normal","date":1643392406,"dismissed":false,"date_dismissed":null,"dismiss_wake":null,"source":"moonlight","feed":"Moonraker"}]}]}"#;

//     let deserialized: MoonResponse = serde_json::from_str(json).unwrap();

//     if let MoonNotification {
//         jsonrpc,
//         method,
//         params,
//     } = deserialized
//     {
//         assert_eq!(jsonrpc, JsonRpcVersion::V2);
//         assert_eq!(method, NotificationMethod::NotifyAnnouncementUpdate);
//         if let Some(NotificationParam::Announcement(param)) = params {
//             assert_eq!(param.entries.len(), 3);
//             // ... assert other fields ...
//         } else {
//             panic!("Invalid params");
//         }
//     } else {
//         panic!("Invalid MoonResponse");
//     }
// }


#[test]
fn test_serialize_announcement_update() {
	let notification = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyAnnouncementUpdate,
		params: Some(NotificationParam::AnnouncementUpdate(AnnouncementUpdateParam {
			entries: vec![
				AnnouncementEntry {
					entry_id: "arksine/moonlight/issue/3".to_string(),
					url: "https://github.com/Arksine/moonlight/issues/3".to_string(),
					title: "Test announcement 3".to_string(),
					description: "Test Description [with a link](https://moonraker.readthedocs.io).".to_string(),
					priority: "normal".to_string(),
					date: 1647459219,
					dismissed: false,
					date_dismissed: None,
					dismiss_wake: None,
					source: "moonlight".to_string(),
					feed: "moonlight".to_string(),
				},
				// Add more entries as needed
			],
		})),
	};

	let serialized = serde_json::to_string(&notification).unwrap();
	assert_eq!(
		serialized,
		r#"{"jsonrpc":"2.0","method":"notify_announcement_update","params":[{"entries":[{"entry_id":"arksine/moonlight/issue/3","url":"https://github.com/Arksine/moonlight/issues/3","title":"Test announcement 3","description":"Test Description [with a link](https://moonraker.readthedocs.io).","priority":"normal","date":1647459219,"dismissed":false,"source":"moonlight","feed":"moonlight"}]}]}"#
	);
}

#[test]
fn test_deserialize_announcement_update() {
	let json = r#"{"jsonrpc":"2.0","method":"notify_announcement_update","params":[{"entries":[{"entry_id":"arksine/moonlight/issue/3","url":"https://github.com/Arksine/moonlight/issues/3","title":"Test announcement 3","description":"Test Description [with a link](https://moonraker.readthedocs.io).","priority":"normal","date":1647459219,"dismissed":false,"source":"moonlight","feed":"moonlight"}]}]}"#;
	let deserialized: MoonNotification = serde_json::from_str(json).unwrap();

	let MoonNotification { jsonrpc, method, params } = deserialized;

	// if let MoonNotification { jsonrpc, method, params } = deserialized {
	assert_eq!(jsonrpc, JsonRpcVersion::V2);
	assert_eq!(method, NotificationMethod::NotifyAnnouncementUpdate);
	if let Some(NotificationParam::AnnouncementUpdate(param)) = params {
		// let param = param[0].clone();
		assert_eq!(param.entries.len(), 1);
		let entry = &param.entries[0];
		assert_eq!(entry.entry_id, "arksine/moonlight/issue/3");
		assert_eq!(entry.url, "https://github.com/Arksine/moonlight/issues/3");
		assert_eq!(entry.title, "Test announcement 3");
		assert_eq!(
			entry.description,
			"Test Description [with a link](https://moonraker.readthedocs.io)."
		);
		assert_eq!(entry.priority, "normal");
		assert_eq!(entry.date, 1647459219);
		assert!(!entry.dismissed);
		assert_eq!(entry.source, "moonlight");
		assert_eq!(entry.feed, "moonlight");
	} else {
		panic!("Invalid params");
	}
	// } else {
	// 	panic!("Invalid MoonResponse");
	// }
}
