use moonsock::{
	MoonMethod,
	// MoonMSG,
	MoonRequest,
};

// mod server;

#[test]
fn test_unknown_method() {
	let json = r#"{
        "jsonrpc": "2.0",
        "method": "unknown_method",
        "params":{
            "limit": 50,
            "start": 10,
            "since": 464.54,
            "before": 1322.54,
            "order": "asc"
        },
        "id": 5656
    }"#;

	// let moon_msg: MoonMSG = serde_json::from_str(json).unwrap();
	let moon_msg: MoonRequest = serde_json::from_str(json).unwrap();
	match moon_msg.method {
		MoonMethod::Other(method) => assert_eq!(method, "unknown_method"),
		_ => panic!("Expected MoonMethod::Other"),
	}
}
