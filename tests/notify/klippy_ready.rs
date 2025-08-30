use moonsock::{MoonNotification, NotificationMethod};
use serde_json::json;

#[test]
fn test_parse_notification_klippy_ready() {
	let json = json!({
		"jsonrpc": "2.0",
		"method": "notify_klippy_ready"
	});
	let notification: MoonNotification = serde_json::from_value(json).unwrap();

	let MoonNotification { method, params, .. } = notification;

	assert_eq!(method, NotificationMethod::NotifyKlippyReady);
	assert_eq!(params, None);

	// match notification {
	// 	MoonNotification { method, params, .. } => {
	// 		assert_eq!(method, NotificationMethod::NotifyKlippyReady);
	// 		assert_eq!(params, None);
	// 	}
	// 	_ => panic!("Invalid response type"),
	// }
}
