use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerHistoryListResponse {
	pub count: u32,
	pub jobs: Vec<ServerHistoryJob>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerHistoryJob {
	pub job_id: String,
	pub exists: bool,
	pub end_time: Option<f64>,
	pub filament_used: Option<f64>,
	pub filename: String,
	pub metadata: serde_json::Value,
	pub print_duration: Option<f64>,
	pub status: String,
	pub start_time: f64,
	pub total_duration: Option<f64>,
}
