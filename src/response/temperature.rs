use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TempStoreData {
	TempTgtsPowers {
		temperatures: Option<Vec<f32>>,
		targets: Vec<f32>,
		powers: Vec<f32>,
	},
	Temp {
		temperatures: Option<Vec<f32>>,
	},
}

/// The names of the items in the temperature store
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum HeaterNames {
	#[serde(rename = "heater_bed")]
	HeaterBed,
	#[serde(rename = "extruder")]
	Extruder,
	#[serde(rename = "extruder1")]
	Extruder1,
	#[serde(rename = "extruder2")]
	Extruder2,
	#[serde(rename = "extruder3")]
	Extruder3,
	#[serde(rename = "extruder4")]
	Extruder4,
	#[serde(rename = "extruder5")]
	Extruder5,
	#[serde(rename = "extruder6")]
	Extruder6,
	#[serde(rename = "extruder7")]
	Extruder7,
	#[serde(rename = "extruder8")]
	Extruder8,
	#[serde(rename = "extruder9")]
	Extruder9,
	#[serde(rename = "extruder10")]
	Extruder10,
	#[serde(rename = "temperature_fan")]
	TemperatureFan,
	#[serde(rename = "temperature_sensor")]
	TemperatureSensor,
	NameStr(String),
}
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TemperatureStore {
	#[serde(flatten)]
	pub items: HashMap<HeaterNames, TempStoreData>,
}

impl TemperatureStore {
	pub fn new() -> Self {
		Self { items: HashMap::new() }
	}
	pub fn add_to_hashmap(&mut self, key: HeaterNames, value: TempStoreData) {
		self.items.insert(key, value);
	}
}

// impl Default for TemperatureStore {
// 	fn default() -> Self {
// 		Self::new()
// 	}
// }

