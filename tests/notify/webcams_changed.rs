use moonsock::{JsonRpcVersion, MoonNotification, NotificationMethod, NotificationParam, Webcam, WebcamsChangedParams};


#[test]
fn test_serialize_notify_webcams_changed() {
	let message = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyWebcamsChanged,
		params: Some(NotificationParam::WebcamsChanged(WebcamsChangedParams {
			webcams: vec![
				Webcam {
					name: "tc2".to_string(),
					location: "printer".to_string(),
					service: "mjpegstreamer".to_string(),
					enabled: true,
					icon: "mdiWebcam".to_string(),
					target_fps: 15,
					target_fps_idle: 5,
					stream_url: "http://printer.lan/webcam?action=stream".to_string(),
					snapshot_url: "http://printer.lan/webcam?action=snapshot".to_string(),
					flip_horizontal: false,
					flip_vertical: false,
					rotation: 0,
					aspect_ratio: "4:3".to_string(),
					extra_data: serde_json::json!({}),
					source: "database".to_string(),
				},
				Webcam {
					name: "TestCam".to_string(),
					location: "printer".to_string(),
					service: "mjpegstreamer".to_string(),
					enabled: true,
					icon: "mdiWebcam".to_string(),
					target_fps: 15,
					target_fps_idle: 5,
					stream_url: "/webcam/?action=stream".to_string(),
					snapshot_url: "/webcam/?action=snapshot".to_string(),
					flip_horizontal: false,
					flip_vertical: false,
					rotation: 0,
					aspect_ratio: "4:3".to_string(),
					extra_data: serde_json::json!({}),
					source: "database".to_string(),
				},
			],
		})),
	};

	let expected_json = r#"{"jsonrpc":"2.0","method":"notify_webcams_changed","params":[{"webcams":[{"name":"tc2","location":"printer","service":"mjpegstreamer","enabled":true,"icon":"mdiWebcam","target_fps":15,"target_fps_idle":5,"stream_url":"http://printer.lan/webcam?action=stream","snapshot_url":"http://printer.lan/webcam?action=snapshot","flip_horizontal":false,"flip_vertical":false,"rotation":0,"aspect_ratio":"4:3","extra_data":{},"source":"database"},{"name":"TestCam","location":"printer","service":"mjpegstreamer","enabled":true,"icon":"mdiWebcam","target_fps":15,"target_fps_idle":5,"stream_url":"/webcam/?action=stream","snapshot_url":"/webcam/?action=snapshot","flip_horizontal":false,"flip_vertical":false,"rotation":0,"aspect_ratio":"4:3","extra_data":{},"source":"database"}]}]}"#;
	let actual_json = serde_json::to_string(&message).unwrap();

	assert_eq!(expected_json, actual_json);
}

#[test]
fn test_deserialize_notify_webcams_changed() {
	let json = r#"{"jsonrpc":"2.0","method":"notify_webcams_changed","params":[{"webcams":[{"name":"tc2","location":"printer","service":"mjpegstreamer","enabled":true,"icon":"mdiWebcam","target_fps":15,"target_fps_idle":5,"stream_url":"http://printer.lan/webcam?action=stream","snapshot_url":"http://printer.lan/webcam?action=snapshot","flip_horizontal":false,"flip_vertical":false,"rotation":0,"aspect_ratio":"4:3","extra_data":{},"source":"database"},{"name":"TestCam","location":"printer","service":"mjpegstreamer","enabled":true,"icon":"mdiWebcam","target_fps":15,"target_fps_idle":5,"stream_url":"/webcam/?action=stream","snapshot_url":"/webcam/?action=snapshot","flip_horizontal":false,"flip_vertical":false,"rotation":0,"aspect_ratio":"4:3","extra_data":{},"source":"database"}]}]}"#;
	let expected_message = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyWebcamsChanged,
		params: Some(NotificationParam::WebcamsChanged(WebcamsChangedParams {
			webcams: vec![
				Webcam {
					name: "tc2".to_string(),
					location: "printer".to_string(),
					service: "mjpegstreamer".to_string(),
					enabled: true,
					icon: "mdiWebcam".to_string(),
					target_fps: 15,
					target_fps_idle: 5,
					stream_url: "http://printer.lan/webcam?action=stream".to_string(),
					snapshot_url: "http://printer.lan/webcam?action=snapshot".to_string(),
					flip_horizontal: false,
					flip_vertical: false,
					rotation: 0,
					aspect_ratio: "4:3".to_string(),
					extra_data: serde_json::json!({}),
					source: "database".to_string(),
				},
				Webcam {
					name: "TestCam".to_string(),
					location: "printer".to_string(),
					service: "mjpegstreamer".to_string(),
					enabled: true,
					icon: "mdiWebcam".to_string(),
					target_fps: 15,
					target_fps_idle: 5,
					stream_url: "/webcam/?action=stream".to_string(),
					snapshot_url: "/webcam/?action=snapshot".to_string(),
					flip_horizontal: false,
					flip_vertical: false,
					rotation: 0,
					aspect_ratio: "4:3".to_string(),
					extra_data: serde_json::json!({}),
					source: "database".to_string(),
				},
			],
		})),
	};

	let actual: MoonNotification = serde_json::from_str(json).unwrap();

	assert_eq!(expected_message, actual);
}
