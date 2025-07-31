// use jsonrpc_message_derive::JsonRpcMessage;
use serde::{Deserialize, Serialize};

use crate::{
	jsonrpc_ws_client::{
		JsonRpcRequest,
		// JsonRpcMessage
	},
	// jsonrpc_ws_client::JsonRpcMessage,
	JsonRpcVersion,
	MoonMethod,
	MoonParam,
};


/// A MoonRequest represents a JSON-RPC request that can be sent to Moonraker via the WebSocket.
///
/// # Examples
///
/// ```rust
/// use moonsock::{MoonRequest, MoonMethod, MoonParam};
///
/// let request = MoonRequest::new(MoonMethod::PrinterGcodeScript, Some(MoonParam::GcodeScript { script: "G28".to_string() }));
///
/// println!("{:?}", request);
/// ```
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonRpcMessage)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoonRequest {
	/// The JSON-RPC version of the request.
	pub jsonrpc: JsonRpcVersion,
	/// The method to be called.
	pub method: MoonMethod,
	/// The parameters to be passed to the method.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub params: Option<MoonParam>,
	/// The ID of the request.
	pub id: u32,
}

// impl JsonRpcMessage for MoonRequest {
//     /// Returns the ID of the request.
//     fn id(&self) -> Option<u32> {
//         Some(self.id)
//     }
//     /// Sets the ID of the request.
//     fn set_id(&mut self, id: u32) {
//         self.id = id;
//     }
// }

impl From<MoonRequest> for JsonRpcRequest {
	fn from(val: MoonRequest) -> Self {
		// let method = serde_json::to_string(&self.method).unwrap();
		// // let params = self.params.map(|p| serde_json::to_value(p).unwrap());
		// let params = if self.params.is_none() {
		//     None
		// } else {
		//     Some(serde_json::to_value(&self.params.unwrap()).unwrap())
		// };

		// println!("Sending message with method: {}", method);
		// println!("Params: {:?}", params);
		// JsonRpcRequest::build(method, params)
		JsonRpcRequest::build(val.method, val.params).unwrap()

		// JsonRpcRequest::build(serde_json::to_string(&self.method).unwrap(), self.params.map(|p| serde_json::to_value(p).unwrap()))
	}
}


impl MoonRequest {
	/// Creates a new MoonRequest which can be sent to Moonraker via the websocket.
	///
	/// The method is the name of the method to be called and is required for all messages.
	///
	/// The params are the parameters to be passed to the method and are optional for some types of messages.
	///
	/// The ID of the message is handled internally and does not need to be provided.
	///
	/// Refer to the Moonraker API docs for more information.
	///
	/// # Examples
	///
	/// ```rust
	/// use moonsock::{MoonRequest, MoonMethod, MoonParam};
	///
	/// let request = MoonRequest::new(MoonMethod::PrinterGcodeScript, Some(MoonParam::GcodeScript { script: "G28".to_string() }));
	///
	/// println!("{:?}", request);
	/// ```
	pub fn new(method: MoonMethod, params: Option<MoonParam>) -> Self {
		let id = rand::random();
		Self { jsonrpc: JsonRpcVersion::V2, method, params, id }
	}

	/// A convenience method for creating a MoonRequest with the method set to `printer.gcode.script` and the params set to the given G-code script.
	///
	/// # Examples
	///
	/// ```rust
	/// use moonsock::MoonRequest;
	///
	/// let request = MoonRequest::gcode("G28".to_string());
	///
	/// println!("{:?}", request);
	/// ```
	pub fn gcode(gcode: String) -> Self {
		Self::new(
			MoonMethod::PrinterGcodeScript,
			Some(MoonParam::GcodeScript { script: gcode.to_string() }),
		)
	}
}
