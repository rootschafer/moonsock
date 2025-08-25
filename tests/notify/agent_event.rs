use moonsock::{
	AgentEventParams, JsonRpcVersion, MoonNotification, MoonResponse, NotificationMethod, NotificationParam,
};

#[test]
fn test_serialize_notify_agent_event() {
	let message = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyAgentEvent,
		params: Some(NotificationParam::AgentEvent(AgentEventParams {
			agent: "moonagent".to_string(),
			event: "connected".to_string(),
			data: Some(serde_json::json!({
				"name": "moonagent",
				"version": "0.0.1",
				"type": "agent",
				"url": "https://github.com/arksine/moontest"
			})),
		})),
	};

	let expected_json = r#"{"jsonrpc":"2.0","method":"notify_agent_event","params":[{"agent":"moonagent","event":"connected","data":{"name":"moonagent","version":"0.0.1","type":"agent","url":"https://github.com/arksine/moontest"}}]}"#;
	let actual_json = serde_json::to_string(&message).unwrap();

	assert_eq!(expected_json, actual_json);
}

#[test]
fn test_deserialize_notify_agent_event() {
	let json = r#"{"jsonrpc":"2.0","method":"notify_agent_event","params":[{"agent":"moonagent","event":"connected","data":{"name":"moonagent","version":"0.0.1","type":"agent","url":"https://github.com/arksine/moontest"}}]}"#;
	let expected_message = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyAgentEvent,
		params: Some(NotificationParam::AgentEvent(AgentEventParams {
			agent: "moonagent".to_string(),
			event: "connected".to_string(),
			data: Some(serde_json::json!({
				"name": "moonagent",
				"version": "0.0.1",
				"type": "agent",
				"url": "https://github.com/arksine/moontest"
			})),
		})),
	};

	// let actual_message: MoonResponse = serde_json::from_str(json).unwrap();
	let actual_message: MoonNotification = serde_json::from_str(json).unwrap();

	assert_eq!(expected_message, actual_message);
}

