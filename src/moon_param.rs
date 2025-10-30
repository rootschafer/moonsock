use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MoonParam {
	None,
	AccessLoginParams {
		username: String,
		password: String,
		source: String,
	},
	AccessPostUserParams {
		username: String,
		password: String,
	},
	TemperatureStoreParams {
		include_monitors: bool,
	},
	ServerConnectionIdentifyParams {
		client_name: String,
		version: String,
		#[serde(rename = "type")]
		client_type: String,
		url: String,
		#[serde(skip_serializing_if = "Option::is_none")]
		access_token: Option<String>,
		#[serde(skip_serializing_if = "Option::is_none")]
		api_key: Option<String>,
	},
	PrinterObjectsQuery {
		objects: PrinterObject,
	},
	PrinterObjectsSubscribe {
		objects: PrinterObject,
	},
	GcodeScript {
		script: String,
	},
	GcodeSubscribeOutput {
		response_template: serde_json::Value,
	},
	ServerFilesPath {
		path: String,
	},
	ServerFilesMoveParams {
		source: String,
		dest: String,
	},
	ServerFilesZipParams {
		items: Vec<String>,
		dest: String,
	},
	ServerJobQueuePostJobParams {
		filename: String,
	},
	ServerJobQueueDeleteJobParams {
		job_id: String,
	},
	Count(u64),
	Filename(String),
	Service(SystemdSevice),
	Password(String),
	Root(String),
	Name(String),
	Refresh(bool),
	Device(String),
	Uuid(u64),
	ButtonEvent {
		name: String,
		typee: String,
		event: Event,
		aux: String,
	},
	ParamVec(Vec<MoonParam>),
	#[serde(untagged)]
	Other(serde_json::Value),
}

impl MoonParam {
	pub fn from_json(json: &str) -> Result<MoonParam, serde_json::Error> {
		serde_json::from_str(json)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PrinterObject {
	#[serde(rename = "gcode_move")]
	GcodeMove(Option<Vec<String>>),
	#[serde(rename = "toolhead")]
	Toolhead(Option<Vec<String>>),
	#[serde(rename = "z_tilt")]
	ZTilt(Option<Vec<String>>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SystemdSevice {
	#[serde(rename = "{klipper}")]
	Klipper,
	#[serde(rename = "{moonraker}")]
	Moonraker,
	#[serde(rename = "{nginx}")]
	Nginx,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
	pub elapsed_time: f32,
	pub received_time: f32,
	pub render_time: f32,
	pub pressed: bool,
}
