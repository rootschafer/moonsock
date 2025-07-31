use moonsock::{
	MoonOk,
	// MoonMSG,
	MoonResponse,
	// response::{MoonResultData, MoonOk},
	MoonResultData,
};
#[test]
fn result_ok() {
	// let message_struct = MoonMSG::MoonResult {
	let message_struct = MoonResponse::MoonResult {
		jsonrpc: moonsock::JsonRpcVersion::V2,
		result: MoonResultData::Ok(MoonOk::Ok),
		id: 1340,
	};
	let message_string = serde_json::to_string(&message_struct).unwrap();
	println!("{message_string}");
	let message = r##"{
        "jsonrpc": "2.0", 
        "result": "ok", 
        "id": 1340
    }"##;
	// let message_parsed: MoonMSG = serde_json::from_str(message).unwrap();
	let message_parsed: MoonResponse = serde_json::from_str(message).unwrap();
	match message_parsed {
		// MoonMSG::MoonResult { result, id, .. } => {
		MoonResponse::MoonResult { result, id, .. } => {
			assert_eq!(result, MoonResultData::Ok(MoonOk::Ok));
			assert_eq!(id, 1340);
		}
		_ => panic!("Wrong message type"),
	}
}

