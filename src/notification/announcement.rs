use serde::{Serialize, Deserialize};

// use crate::utils::single_element_array;

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct AnnouncementEntry {
//     pub entry_id: String,
//     pub url: String,
//     pub title: String,
//     pub description: String,
//     pub priority: String,
//     pub date: i64,
//     pub dismissed: bool,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     pub date_dismissed: Option<i64>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     pub dismiss_wake: Option<i64>,
//     pub source: String,
//     pub feed: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct AnnouncementParams {
//     #[serde(with = "single_element_array")]
//     pub entries: Vec<AnnouncementEntry>,
// }


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnnouncementEntry {
    pub entry_id: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub priority: String,
    pub date: i64,
    pub dismissed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_dismissed: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismiss_wake: Option<i64>,
    pub source: String,
    pub feed: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnnouncementUpdateParam {
    // #[serde(with = "single_element_array")]
    pub entries: Vec<AnnouncementEntry>,
}


// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct AnnouncementDismissedParam {
//     pub entry_id: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct AnnouncementWakeParam {
//     pub entry_id: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntryId {
    pub entry_id: String,
}