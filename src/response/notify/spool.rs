use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActiveSpoolSetParams {
	pub spool_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpoolmanStatusChangedParams {
	pub spoolman_connected: bool,
}
