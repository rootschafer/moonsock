use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
	utils::single_element_array, ActiveSpoolSetParams, AnnouncementUpdateParam, CpuThrottledState, EntryId,
	FilelistChangedParam, HistoryChangedParam, JsonRpcNotification, JsonRpcVersion, NotifyProcStatUpdateParam,
	PrinterObjectStatus, ServiceState, SpoolmanStatusChangedParams, WebcamsChangedParams,
};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoonNotification {
	pub jsonrpc: JsonRpcVersion,
	pub method: NotificationMethod,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub params: Option<NotificationParam>,
}

impl Default for MoonNotification {
	fn default() -> Self {
		Self {
			jsonrpc: JsonRpcVersion::V2,
			method: NotificationMethod::Other("not_a_real_notification_method".to_string()),
			params: None,
		}
	}
}

impl TryFrom<JsonRpcNotification> for MoonNotification {
	type Error = Box<dyn std::error::Error + Send + Sync>;
	fn try_from(value: JsonRpcNotification) -> Result<Self, Self::Error> {
		// Parse the method string directly by converting it to JSON and then deserializing
		let method_json = format!("\"{}\"", value.method);
		match serde_json::from_str(&method_json) {
			Ok(method) => match value.params {
				Some(params) => match serde_json::from_value(params) {
					Ok(params) => Ok(MoonNotification { method, params: Some(params), ..Default::default() }),
					Err(e) => Err(format!("Error parsing notification params: {}", e).into()),
				},
				None => Ok(MoonNotification { method, ..Default::default() }),
			},
			Err(e) => Err(format!("Error parsing notification method: {}", e).into()),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationMethod {
	#[serde(rename = "notify_gcode_response")]
	NotifyGcodeResponse,
	#[serde(rename = "notify_status_update")]
	NotifyStatusUpdate,
	#[serde(rename = "notify_klippy_ready")]
	NotifyKlippyReady,
	#[serde(rename = "notify_klippy_shutdown")]
	NotifyKlippyShutdown,
	#[serde(rename = "notify_klippy_disconnected")]
	NotifyKlippyDisconnected,
	#[serde(rename = "notify_filelist_changed")]
	NotifyFilelistChanged,
	#[serde(rename = "notify_update_response")]
	NotifyUpdateResponse,
	#[serde(rename = "notify_update_refreshed")]
	NotifyUpdateRefreshed,
	#[serde(rename = "notify_cpu_throttled")]
	NotifyCpuThrottled,
	#[serde(rename = "notify_proc_stat_update")]
	NotifyProcStatUpdate,
	#[serde(rename = "notify_history_changed")]
	NotifyHistoryChanged,
	#[serde(rename = "notify_user_created")]
	NotifyUserCreated,
	#[serde(rename = "notify_user_deleted")]
	NotifyUserDeleted,
	#[serde(rename = "notify_user_logged_out")]
	NotifyUserLoggedOut,
	#[serde(rename = "notify_service_state_changed")]
	NotifyServiceStateChanged,
	#[serde(rename = "notify_job_queue_changed")]
	NotifyJobQueueChanged,
	#[serde(rename = "notify_button_event")]
	NotifyButtonEvent,
	#[serde(rename = "notify_announcement_update")]
	NotifyAnnouncementUpdate,
	#[serde(rename = "notify_announcement_dismissed")]
	NotifyAnnouncementDismissed,
	#[serde(rename = "notify_announcement_wake")]
	NotifyAnnouncementWake,
	#[serde(rename = "notify_sudo_alert")]
	NotifySudoAlert,
	#[serde(rename = "notify_webcams_changed")]
	NotifyWebcamsChanged,
	#[serde(rename = "notify_active_spool_set")]
	NotifyActiveSpoolSet,
	#[serde(rename = "notify_spoolman_status_changed")]
	NotifySpoolmanStatusChanged,
	#[serde(rename = "notify_agent_event")]
	NotifyAgentEvent,
	Other(String),
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum NotificationParam {
	String(Vec<String>),
	Float(Vec<f64>),
	StatusUpdate(PrinterObjectStatus, f64),
	FilelistChanged(Vec<FilelistChangedParam>),
	UpdateResponse(Vec<UpdateResponseParam>),
	UpdateRefreshed(Vec<UpdateRefreshedParam>),
	CpuThrottled(Vec<CpuThrottledState>),
	ProcStatUpdate(Vec<NotifyProcStatUpdateParam>),
	HistoryChanged(Vec<HistoryChangedParam>),
	/// `params` field for `notify_user_created`, `notify_user_deleted`, and `notify_user_logged_out`
	User(Vec<UserParam>),
	ServiceStateChanged(Vec<ServiceStateChangedParam>),
	JobQueueChanged(Vec<JobQueueChangedParam>),
	#[serde(with = "single_element_array")]
	ButtonEvent(ButtonEventParam),
	#[serde(with = "single_element_array")]
	AnnouncementUpdate(AnnouncementUpdateParam),
	#[serde(with = "single_element_array")]
	AnnouncementEntryId(EntryId),
	#[serde(with = "single_element_array")]
	SudoAlert(SudoAlertParams),
	#[serde(with = "single_element_array")]
	WebcamsChanged(WebcamsChangedParams),
	#[serde(with = "single_element_array")]
	ActiveSpoolSet(ActiveSpoolSetParams),
	#[serde(with = "single_element_array")]
	SpoolmanStatusChanged(SpoolmanStatusChangedParams),
	#[serde(with = "single_element_array")]
	AgentEvent(AgentEventParams),
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct NotifyProcStatUpdateParam {
//     pub moonraker_stats: MoonrakerStats,
//     pub cpu_temp: Option<f64>,
//     pub network: HashMap<String, NotifyProcStatUpdateNetworkItem>,
//     pub system_cpu_usage: HashMap<String, f64>,
//     pub websocket_connections: u64,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct NotifyProcStatUpdateNetworkItem {
//     pub rx_bytes: u64,
//     pub tx_bytes: u64,
//     pub bandwidth: f64,
// }


// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct FilelistChangedParam {
//     pub action: FilelistAction,
//     pub item: FilelistItem,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub source_item: Option<FilelistItem>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub enum FilelistAction {
//     #[serde(rename = "create_file")]
//     CreateFile,
//     #[serde(rename = "create_dir")]
//     CreateDir,
//     #[serde(rename = "delete_file")]
//     DeleteFile,
//     #[serde(rename = "delete_dir")]
//     DeleteDir,
//     #[serde(rename = "move_file")]
//     MoveFile,
//     #[serde(rename = "move_dir")]
//     MoveDir,
//     #[serde(rename = "modify_file")]
//     ModifyFile,
//     #[serde(rename = "root_update")]
//     RootUpdate,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct FilelistItem {
//     pub path: String,
//     pub root: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub size: Option<u64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub modified: Option<u64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub permissions: Option<String>,
// }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateResponseParam {
	pub application: String,
	pub proc_id: u64,
	pub message: String,
	pub complete: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateRefreshedParam {
	pub busy: bool,
	pub github_rate_limit: u64,
	pub github_requests_remaining: u64,
	pub github_limit_reset_time: u64,
	pub version_info: VersionInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VersionInfo {
	pub system: SystemPkgInfo,
	#[serde(flatten)]
	pub applications: HashMap<String, ApplicationInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemPkgInfo {
	pub package_count: u64,
	pub package_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ApplicationInfo {
	GitRepo(GitRepoInfo),
	Web(WebInfo),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitRepoInfo {
	pub channel: String,
	pub debug_enabled: bool,
	pub is_valid: bool,
	pub configured_type: String,
	pub corrupt: bool,
	pub info_tags: Vec<String>,
	pub detected_type: String,
	pub remote_alias: String,
	pub branch: String,
	pub owner: String,
	pub repo_name: String,
	pub version: String,
	pub remote_version: String,
	pub rollback_version: String,
	pub current_hash: String,
	pub remote_hash: String,
	pub is_dirty: bool,
	pub detached: bool,
	pub commits_behind: Vec<CommitInfo>,
	pub git_messages: Vec<String>,
	pub full_version_string: String,
	pub pristine: bool,
	pub recovery_url: String,
	pub remote_url: String,
	pub warnings: Vec<String>,
	pub anomalies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebInfo {
	pub name: String,
	pub owner: String,
	pub version: String,
	pub remote_version: String,
	pub rollback_version: String,
	pub configured_type: String,
	pub channel: String,
	pub info_tags: Vec<String>,
	pub warnings: Vec<String>,
	pub anomalies: Vec<String>,
	pub is_valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommitInfo {
	pub sha: String,
	pub author: String,
	pub date: String,
	pub subject: String,
	pub message: String,
	pub tag: Option<String>,
}

/// Used for `notify_user_created`, `notify_user_deleted`, and `notify_user_logged_out`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserParam {
	pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServiceStateChangedParam {
	#[serde(flatten)]
	pub services: HashMap<String, ServiceState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobQueueChangedParam {
	pub action: JobQueueAction,
	pub updated_queue: Option<Vec<String>>,
	pub queue_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JobQueueAction {
	#[serde(rename = "state_changed")]
	StateChanged,
	#[serde(rename = "jobs_added")]
	JobsAdded,
	#[serde(rename = "jobs_removed")]
	JobsRemoved,
	#[serde(rename = "job_loaded")]
	JobLoaded,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ButtonEventParam {
	pub name: String,
	#[serde(rename = "type")]
	pub button_type: String,
	pub event: ButtonEventDetails,
	pub aux: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ButtonEventDetails {
	pub elapsed_time: f64,
	pub received_time: f64,
	pub render_time: f64,
	pub pressed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SudoAlertParams {
	pub sudo_requested: bool,
	pub sudo_messages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentEventParams {
	pub agent: String,
	pub event: String,
	pub data: Option<serde_json::Value>,
}
