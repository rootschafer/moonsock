use moonsock::MoonNotification;
use moonsock::{
	ButtonEventDetails,
	ButtonEventParam,
	// ButtonEventParams,
	JsonRpcVersion,
	MoonResponse,
	NotificationMethod,
	NotificationParam,
};

// #[test]
// fn test_serialize_button_event() {
//     let notification = MoonNotification {
//         jsonrpc: JsonRpcVersion::V2,
//         method: NotificationMethod::NotifyButtonEvent,
//         params: Some(NotificationParam::ButtonEvent(ButtonEventNotification {
//             param: ButtonEventParam {
//                 name: "my_button".to_string(),
//                 button_type: "gpio".to_string(),
//                 event: ButtonEventDetails {
//                     elapsed_time: 0.09323832602240145,
//                     received_time: 698614.214597004,
//                     render_time: 698614.214728513,
//                     pressed: false,
//                 },
//                 aux: None,
//             },
//         })),
//     };

//     let expected_json = r#"{
//         "jsonrpc": "2.0",
//         "method": "notify_button_event",
//         "params": {
//             "name": "my_button",
//             "type": "gpio",
//             "event": {
//                 "elapsed_time": 0.09323832602240145,
//                 "received_time": 698614.214597004,
//                 "render_time": 698614.214728513,
//                 "pressed": false
//             },
//             "aux": null
//         }
//     }"#;

//     let actual_json = serde_json::to_string_pretty(&notification).unwrap();
//     assert_eq!(actual_json, expected_json);
// }

// #[test]
// fn test_deserialize_button_event() {
//     let json = r#"{
//         "jsonrpc": "2.0",
//         "method": "notify_button_event",
//         "params": {
//             "name": "my_button",
//             "type": "gpio",
//             "event": {
//                 "elapsed_time": 0.09323832602240145,
//                 "received_time": 698614.214597004,
//                 "render_time": 698614.214728513,
//                 "pressed": false
//             },
//             "aux": null
//         }
//     }"#;

//     let notification: MoonResponse = serde_json::from_str(json).unwrap();

//     match notification {
//         MoonNotification {
//             method: NotificationMethod::NotifyButtonEvent,
//             params: Some(NotificationParam::ButtonEvent(notification)),
//             ..
//         } => {
//             assert_eq!(notification.param.name, "my_button");
//             assert_eq!(notification.param.button_type, "gpio");
//             assert_eq!(notification.param.event.elapsed_time, 0.09323832602240145);
//             assert_eq!(notification.param.event.received_time, 698614.214597004);
//             assert_eq!(notification.param.event.render_time, 698614.214728513);
//             assert_eq!(notification.param.event.pressed, false);
//             assert_eq!(notification.param.aux, None);
//         }
//         _ => panic!("Invalid notification"),
//     }
// }

// #[test]
// fn test_serialize_button_event() {
//     let notification = MoonNotification {
//         jsonrpc: JsonRpcVersion::V2,
//         method: NotificationMethod::NotifyButtonEvent,
//         params: Some(NotificationParam::ButtonEvent(ButtonEventParams(ButtonEventParam {
//             name: "my_button".to_string(),
//             button_type: "gpio".to_string(),
//             event: ButtonEventDetails {
//                 elapsed_time: 0.09323832602240145,
//                 received_time: 698614.214597004,
//                 render_time: 698614.214728513,
//                 pressed: false,
//             },
//             aux: None,
//         }))),
//         // params: Some(NotificationParam::ButtonEvent(ButtonEventParam {
//         //     name: "my_button".to_string(),
//         //     button_type: "gpio".to_string(),
//         //     event: ButtonEventDetails {
//         //         elapsed_time: 0.09323832602240145,
//         //         received_time: 698614.214597004,
//         //         render_time: 698614.214728513,
//         //         pressed: false,
//         //     },
//         //     aux: None,
//         // })),
//     };

//     let serialized = serde_json::to_string(&notification).unwrap();
//     assert_eq!(
//         serialized,
//         r#"{"jsonrpc":"2.0","method":"notify_button_event","params":[{"name":"my_button","type":"gpio","event":{"elapsed_time":0.09323832602240145,"received_time":698614.214597004,"render_time":698614.214728513,"pressed":false},"aux":null}]}"#
//     );
// }

// #[test]
// fn test_deserialize_button_event() {
//     let json = r#"{"jsonrpc":"2.0","method":"notify_button_event","params":[{"name":"my_button","type":"gpio","event":{"elapsed_time":0.0932383260224014,"received_time":698614.214597004,"render_time":698614.214728513,"pressed":false},"aux":null}]}"#;
//     let deserialized: MoonResponse = serde_json::from_str(json).unwrap();

//     if let MoonNotification {
//         jsonrpc,
//         method,
//         params,
//     } = deserialized
//     {
//         assert_eq!(jsonrpc, JsonRpcVersion::V2);
//         assert_eq!(method, NotificationMethod::NotifyButtonEvent);
//         if let Some(NotificationParam::ButtonEvent(ButtonEventParams(param))) = params {
//         // if let Some(NotificationParam::ButtonEvent(param)) = params {
//             assert_eq!(param.name, "my_button");
//             assert_eq!(param.button_type, "gpio");
//             assert_eq!(param.event.elapsed_time, 0.0932383260224014);
//             assert_eq!(param.event.received_time, 698614.214597004);
//             assert_eq!(param.event.render_time, 698614.214728513);
//             assert!(!param.event.pressed);
//             assert!(param.aux.is_none());
//         } else {
//             panic!("Invalid params");
//         }
//     } else {
//         panic!("Invalid MoonResponse");
//     }
// }

// #[test]
// fn test_serialize_button_event() {
//     let notification = MoonNotification {
//         jsonrpc: JsonRpcVersion::V2,
//         method: NotificationMethod::NotifyButtonEvent,
//         params: Some(NotificationParam::ButtonEvent(ButtonEventParams(ButtonEventParam {
//             name: "my_button".to_string(),
//             button_type: "gpio".to_string(),
//             event: ButtonEventDetails {
//                 elapsed_time: 0.09323832602240145,
//                 received_time: 698614.214597004,
//                 render_time: 698614.214728513,
//                 pressed: false,
//             },
//             aux: None,
//         }))),
//     };

//     let serialized = serde_json::to_string(&notification).unwrap();
//     assert_eq!(
//         serialized,
//         r#"{"jsonrpc":"2.0","method":"notify_button_event","params":[{"name":"my_button","type":"gpio","event":{"elapsed_time":0.09323832602240145,"received_time":698614.214597004,"render_time":698614.214728513,"pressed":false},"aux":null}]}"#
//     );
// }

// #[test]
// fn test_deserialize_button_event() {
//     let json = r#"{"jsonrpc":"2.0","method":"notify_button_event","params":[{"name":"my_button","type":"gpio","event":{"elapsed_time":0.09323832602240145,"received_time":698614.214597004,"render_time":698614.214728513,"pressed":false},"aux":null}]}"#;
//     let deserialized: MoonResponse = serde_json::from_str(json).unwrap();

//     if let MoonNotification {
//         jsonrpc,
//         method,
//         params,
//     } = deserialized
//     {
//         assert_eq!(jsonrpc, JsonRpcVersion::V2);
//         assert_eq!(method, NotificationMethod::NotifyButtonEvent);
//         if let Some(NotificationParam::ButtonEvent(ButtonEventParams(param))) = params {
//             assert_eq!(param.name, "my_button");
//             assert_eq!(param.button_type, "gpio");
//             assert_eq!(param.event.elapsed_time, 0.09323832602240145);
//             assert_eq!(param.event.received_time, 698614.214597004);
//             assert_eq!(param.event.render_time, 698614.214728513);
//             assert!(!param.event.pressed);
//             assert!(param.aux.is_none());
//         } else {
//             panic!("Invalid params");
//         }
//     } else {
//         panic!("Invalid MoonResponse");
//     }
// }


#[test]
fn test_serialize_button_event() {
	let notification = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyButtonEvent,
		// params: Some(NotificationParam::ButtonEvent(ButtonEventParams(ButtonEventParam {
		params: Some(NotificationParam::ButtonEvent(ButtonEventParam {
			name: "my_button".to_string(),
			button_type: "gpio".to_string(),
			event: ButtonEventDetails {
				elapsed_time: 0.09323832602240145,
				received_time: 698614.214597004,
				render_time: 698614.214728513,
				pressed: false,
			},
			aux: None,
		})),
	};

	let serialized = serde_json::to_string(&notification).unwrap();
	assert_eq!(
		serialized,
		r#"{"jsonrpc":"2.0","method":"notify_button_event","params":[{"name":"my_button","type":"gpio","event":{"elapsed_time":0.09323832602240145,"received_time":698614.214597004,"render_time":698614.214728513,"pressed":false},"aux":null}]}"#
	);
}

#[test]
fn test_deserialize_button_event() {
	let json = r#"{"jsonrpc":"2.0","method":"notify_button_event","params":[{"name":"my_button","type":"gpio","event":{"elapsed_time":0.09323832602240145,"received_time":698614.214597004,"render_time":698614.214728513,"pressed":false},"aux":null}]}"#;
	let deserialized: MoonNotification = serde_json::from_str(json).unwrap();

	if let MoonNotification { jsonrpc, method, params } = deserialized {
		assert_eq!(jsonrpc, JsonRpcVersion::V2);
		assert_eq!(method, NotificationMethod::NotifyButtonEvent);
		if let Some(NotificationParam::ButtonEvent(param)) = params {
			assert_eq!(param.name, "my_button");
			assert_eq!(param.button_type, "gpio");
			// assert_eq!(param.event.elapsed_time, 0.09323832602240145);
			assert!((param.event.elapsed_time - 0.09323832602240145).abs() < 1e-10);
			assert_eq!(param.event.received_time, 698614.214597004);
			assert_eq!(param.event.render_time, 698614.214728513);
			assert!(!param.event.pressed);
			assert!(param.aux.is_none());
		} else {
			panic!("Invalid params");
		}
	} else {
		panic!("Invalid MoonResponse");
	}
}

