use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::response::MoonrakerStats;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NotifyProcStatUpdateParam {
	pub moonraker_stats: MoonrakerStats,
	pub cpu_temp: Option<f64>,
	// pub network: HashMap<String, NetworkDetails>,
	// pub system_cpu_usage: HashMap<String, f64>,
	pub network: BTreeMap<String, NetworkDetails>,
	pub system_cpu_usage: BTreeMap<String, f64>,
	pub websocket_connections: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkDetails {
	pub rx_bytes: u64,
	pub tx_bytes: u64,
	pub bandwidth: f64,
}

