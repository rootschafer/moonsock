// use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// use crate::moon_result::PrinterObjectStatus;
// use crate::PrinterObject;

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
    // NotifyProcStatUpdate(Vec<NotifyProcStatUpdateParam>),
    // NotifyCpuThrottled(Vec<CpuThrottledState>),
    // NotifyGcodeResponse( Vec<String> ),
    GcodeScript {
        script: String,
    },
    // Files
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
    // Job Queue
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
    // VecParams(Vec<MoonParam>),
    ParamVec(Vec<MoonParam>),
    // JsonValue(serde_json::Value),
    // MachineProcStats(MachineProcStats),
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
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct MachineProcStats {
//     pub moonraker_stats: Vec<MoonrakerStats>,
//     pub throttled_state: CpuThrottledState,
//     pub cpu_temp: Option<f64>,
//     pub network: HashMap<String, Network>,
//     // pub system_cpu_usage: SystemCpuUsage,
//     pub system_cpu_usage: HashMap<String, f64>,
//     // pub system_memory: SystemMemory,
//     pub system_uptime: Option<f64>,
//     pub websocket_connections: u64,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct CpuThrottledState {
//     pub bits: u64,
//     pub flags: Vec<String>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct TemperatureStoreParams {
//     include_monitors: bool,
// }


// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct MoonrakerStats {
//     pub time: f64,
//     pub cpu_usage: f64,
//     pub memory: u64,
//     pub mem_units: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct Network {
//     pub lo: NetworkData,
//     pub eth0: NetworkData,
//     pub wlan0: NetworkData,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct NetworkData {
//     pub rx_bytes: u64,
//     pub tx_bytes: u64,
//     pub rx_packets: u64,
//     pub tx_packets: u64,
//     pub rx_errs: u64,
//     pub tx_errs: u64,
//     pub rx_drop: u64,
//     pub tx_drop: u64,
//     pub bandwidth: f64,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct SystemCpuUsage {
//     pub cpu: f64,
//     pub cpu0: f64,
//     pub cpu1: f64,
//     pub cpu2: f64,
//     pub cpu3: f64,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct SystemMemory {
//     pub total: u64,
//     pub available: u64,
//     pub used: u64,
// }

