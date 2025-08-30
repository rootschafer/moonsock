// use moonsock::MoonMSG;
use moonsock::MoonResponse;

#[test]
fn must_home_axis_first() {
	let message = r##"{
        "jsonrpc": "2.0", 
        "error": {
            "code": 400, 
            "message": "{'message': 'Must home axis first: 156.600 210.000 50.025 [7013.719]', 'error': 'WebRequestError'}"
        }, 
        "id": 1340
    }"##;

	let msg: MoonResponse = serde_json::from_str(message).unwrap();
	// println!("{:?}", msg);
	// let msg_string = serde_json::to_string(&msg).unwrap();
	// println!("{}", msg_string);
	match msg {
		// MoonMSG::MoonError { error, id, .. } => {
		MoonResponse::MoonError { error, id, .. } => {
			// println!("Error {}: {}", error.code, message);
			assert_eq!(error.code, 400);
			assert_eq!(id, Some(1340));
			assert_eq!(
				error.message,
				"{'message': 'Must home axis first: 156.600 210.000 50.025 [7013.719]', 'error': 'WebRequestError'}"
			);
		}
		_ => panic!("Wrong message type"),
	}
}
