use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerJobQueueStatusResponse {
    pub queued_jobs: Vec<ServerJobQueueJob>,
    pub queue_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerJobQueueJob {
    pub filename: String,
    pub name: String,
    pub time_added: f64,
    pub time_in_queue: f64,
}
