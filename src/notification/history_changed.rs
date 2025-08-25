use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HistoryChangedParam {
	pub action: String, // Can be "added" or "finished"
	pub job: JobObject,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobObject {
	pub job_id: String,
	pub exists: bool,
	pub end_time: f64,
	pub filament_used: f64,
	pub filename: String,
	pub metadata: serde_json::Value, // Assuming metadata is arbitrary JSON
	pub print_duration: f64,
	pub status: String,
	pub start_time: f64,
	pub total_duration: f64,
}
