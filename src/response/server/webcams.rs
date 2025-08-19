use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerWebcamsListResponse {
    pub webcams: Vec<ServerWebcam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerWebcam {
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
}
