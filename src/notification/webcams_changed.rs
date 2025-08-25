use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebcamsChangedParams {
	pub webcams: Vec<Webcam>,
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct Webcam {
//     pub name: String,
//     pub location: String,
//     pub service: String,
//     pub enabled: bool,
//     pub icon: String,
//     pub target_fps: i32,
//     pub target_fps_idle: i32,
//     pub stream_url: String,
//     pub snapshot_url: String,
//     pub flip_horizontal: bool,
//     pub flip_vertical: bool,
//     pub rotation: i32,
//     pub aspect_ratio: String,
//     pub extra_data: std::collections::HashMap<String, serde_json::Value>,
//     pub source: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Webcam {
	pub name: String,
	pub location: String,
	pub service: String,
	pub enabled: bool,
	pub icon: String,
	pub target_fps: u32,
	pub target_fps_idle: u32,
	pub stream_url: String,
	pub snapshot_url: String,
	pub flip_horizontal: bool,
	pub flip_vertical: bool,
	pub rotation: u32,
	pub aspect_ratio: String,
	pub extra_data: serde_json::Value,
	pub source: String,
}

