use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerAnnouncementsResponse {
    pub entries: Vec<ServerAnnouncementEntry>,
    pub feeds: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerAnnouncementEntry {
    pub entry_id: String,
    pub title: String,
    pub description: String,
    pub url: Option<String>,
    pub source: String,
    pub date_published: f64,
    pub date_updated: f64,
}
