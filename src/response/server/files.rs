use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerFileEntry {
	pub path: String,
	pub modified: f64,
	pub size: u64,
	pub permissions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerFilesListResponse(pub Vec<ServerFileEntry>);
