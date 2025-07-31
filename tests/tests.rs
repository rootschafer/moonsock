// use moonsock::MoonMSG;
use moonsock::MoonRequest;

#[test]
fn identify_connection() {
	let message = "{\"jsonrpc\":\"2.0\",\"method\":\"server.connection.identify\",\"params\":{\"client_name\":\"moontest\",\"version\":\"0.0.1\",\"type\":\"web\",\"url\":\"http://github.com/arksine/moontest\"},\"id\":4656}";
	// let serialized: MoonMSG = serde_json::from_str(&message).unwrap();
	let serialized: MoonRequest = serde_json::from_str(message).unwrap();
	let deserialized = serde_json::to_string(&serialized).unwrap();
	// println!("{:?}", &deserialized);
	assert_eq!(message, deserialized);
}

#[test]
fn printer_restart() {
	let message = "{\"jsonrpc\":\"2.0\",\"method\":\"printer.restart\",\"id\":4894}";
	// let serialized: MoonMSG = serde_json::from_str(&message).unwrap();
	let serialized: MoonRequest = serde_json::from_str(message).unwrap();
	let deserialized = serde_json::to_string(&serialized).unwrap();
	// println!("{:?}", &deserialized);
	assert_eq!(message, deserialized);
}

#[test]
fn query_endstops() {
	let message = "{\"jsonrpc\":\"2.0\",\"method\":\"printer.query_endstops.status\",\"id\":3456}";
	// let serialized: MoonMSG = serde_json::from_str(&message).unwrap();
	let serialized: MoonRequest = serde_json::from_str(message).unwrap();
	let deserialized = serde_json::to_string(&serialized).unwrap();
	assert_eq!(message, deserialized);
}

// #[test]
// fn empty() {
//     let msg_struct = MoonMSG::Empty;
//     let msg_struct_string = serde_json::to_string(&msg_struct).unwrap();
//     println!("{}", msg_struct_string);
//     let message = "";
//     let serialized: MoonMSG = serde_json::from_str(&message).unwrap();
//     assert_eq!(serialized, MoonMSG::Empty);
// }
