use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MoonMethod {
	Empty,
	#[serde(rename = "access.login")]
	AccessLogin,
	#[serde(rename = "access.logout")]
	AccessLogout,
	#[serde(rename = "access.post_user")]
	AccessPostUser,
	#[serde(rename = "server.connection.identify")]
	ServerConnectionIdentify,
	#[serde(rename = "server.info")]
	ServerInfo,
	#[serde(rename = "server.config")]
	ServerConfig,
	#[serde(rename = "server.temperature_store")]
	ServerTemperatureStore,
	#[serde(rename = "server.gcode_store")]
	GcodeStore,
	#[serde(rename = "server.restart")]
	ServerRestart,
	#[serde(rename = "printer.query_endstops.status")]
	PrinterQueryEndstopsStatus,
	/// `server.websocket.id` is depreciated by moonraker
	// #[deprecated = "Depreciated by moonraker. Please use the `ServerConnectionIdentify` method/endpoint to retrieve the Websocket's UID"]
	// #[serde(rename="server.websocket.id")]
	// WebsocketID,
	#[serde(rename = "printer.info")]
	PrinterInfo,
	#[serde(rename = "printer.emergency_stop")]
	PrinterEmergencyStop,
	#[serde(rename = "printer.restart")]
	PrinterRestart,
	#[serde(rename = "printer.firmware_restart")]
	PrinterFirmwareRestart,
	/// Lists available printer objects
	#[serde(rename = "printer.objects.list")]
	PrinterObjectsList,
	/// Gets the printer objects.
	#[serde(rename = "printer.objects.query")]
	PrinterObjectsQuery,
	#[serde(rename = "printer.objects.subscribe")]
	PrinterObjectsSubscribe,
	#[serde(rename = "printer.gcode.script")]
	PrinterGcodeScript,
	#[serde(rename = "printer.gcode.help")]
	PrinterGcodeHelp,
	#[serde(rename = "printer.print.start")]
	PrinterPrintStart,
	#[serde(rename = "printer.print.pause")]
	PrinterPrintPause,
	#[serde(rename = "printer.print.resume")]
	PrinterPrintResume,
	#[serde(rename = "printer.print.cancel")]
	PrinterPrintCancel,
	#[serde(rename = "machine.system_info")]
	MachineSystemInfo,
	#[serde(rename = "machine.shutdown")]
	MachineShutdown,
	#[serde(rename = "machine.reboot")]
	MachineReboot,
	#[serde(rename = "machine.services.restart")]
	/// Restarts a system service via sudo systemctl restart {name}. Currently the
	/// moonraker, klipper, MoonCord, KlipperScreen and webcamd services are supported.h
	MachineServicesRestart,
	#[serde(rename = "machine.services.stop")]
	/// Stops a system service via sudo systemctl stop <name>. Currently only webcamd
	/// and klipper are supported.
	MachineServicesStop,
	#[serde(rename = "machine.services.start")]
	/// Starts a system service via sudo systemctl start <name>. Currently only
	/// webcamd and klipper are supported.
	MachineServicesStart,
	/// Returns system usage information about the moonraker process.
	#[serde(rename = "machine.proc_stat")]
	MachineProcStat,
	/// Checks if Moonraker has permission to run commands as root.
	#[serde(rename = "machine.sudo")]
	MachineSudo,
	#[serde(rename = "machine.sudo.password")]
	/// Sets/updates the sudo password currently used by Moonraker. When the password
	/// is set using this endpoint the change is not persistent across restarts.
	MachineSudoPassword,
	#[serde(rename = "server.files.list")]
	ServerFilesList,
	#[serde(rename = "server.files.metadata")]
	ServerFilesMetadata,
	#[serde(rename = "server.files.get_directory")]
	ServerFilesGetDirectory,
	/// Creates a directory at the specified path.
	#[serde(rename = "server.files.post_directory")]
	ServerFilesPostDirectory,
	#[serde(rename = "server.files.delete_directory")]
	ServerFilesDeleteDirectory,
	#[serde(rename = "server.files.move")]
	ServerFilesMove,
	#[serde(rename = "server.files.copy")]
	ServerFilesCopy,
	/// Creates a zip archive from items
	#[serde(rename = "server.files.zip")]
	ServerFilesZip,
	#[serde(rename = "server.files.delete_file")]
	ServerFilesDeleteFile,
	#[serde(rename = "server.database.list")]
	ServerDatabaseList,
	#[serde(rename = "server.database.get_item")]
	ServerDatabaseGetItem,
	#[serde(rename = "server.database.post_item")]
	ServerDatabasePostItem,
	#[serde(rename = "server.database.delete_item")]
	ServerDatabaseDeleteItem,
	#[serde(rename = "server.job_queue.status")]
	ServerJobQueueStatus,
	#[serde(rename = "server.job_queue.post_job")]
	ServerJobQueuePostJob,
	/// Remove a job from the queue (legacy or alt spelling)
	#[serde(rename = "server.job_queue.remove_job")]
	ServerJobQueueRemoveJob,
	#[serde(rename = "server.job_queue.delete_job")]
	ServerJobQueueDeleteJob,
	#[serde(rename = "server.job_queue.clear")]
	ServerJobQueueClear,
	#[serde(rename = "server.job_queue.pause")]
	ServerJobQueuePause,
	#[serde(rename = "server.job_queue.start")]
	ServerJobQueueStart,
	#[serde(rename = "server.announcements.list")]
	ServerAnnouncementsList,
	#[serde(rename = "server.announcements.update")]
	ServerAnnouncementsUpdate,
	#[serde(rename = "server.announcements.dismiss")]
	ServerAnnouncementsDismiss,
	#[serde(rename = "server.announcements.feeds")]
	ServerAnnouncementsFeeds,
	#[serde(rename = "server.announcements.post_feed")]
	ServerAnnouncementsPostFeed,
	#[serde(rename = "server.announcements.delete_feed")]
	ServerAnnouncements,
	#[serde(rename = "server.webcams.list")]
	ServerWebcamsList,
	#[serde(rename = "server.webcams.get_item")]
	ServerWebcamsGetItem,
	#[serde(rename = "server.webcams.delete_item")]
	ServerWebcamsDeleteItem,
	#[serde(rename = "server.webcams.test")]
	ServerWebcamsTest,
	#[serde(rename = "machine.update.status")]
	MachineUpdateStatus,
	#[serde(rename = "machine.update.full")]
	MachineUpdateFull,
	#[serde(rename = "machine.update.moonraker")]
	MachineUpdateMoonraker,
	#[serde(rename = "machine.update.klipper")]
	MachineUpdateKlipper,
	#[serde(rename = "machine.update.client")]
	MachineUpdateClient,
	#[serde(rename = "machine.update.system")]
	MachineUpdateSystem,
	#[serde(rename = "machine.update.recover")]
	MachineUpdateRecover,
	#[serde(rename = "machine.device_power.devices")]
	MachineDevicePowerDevices,
	#[serde(rename = "machine.device_power.get_device")]
	MachineDevicePowerGetDevice,
	#[serde(rename = "machine.device_power.post_device")]
	MachineDevicePowerPostDevice,
	#[serde(rename = "machine.device_power.status")]
	MachineDevicePowerStatus,
	#[serde(rename = "machine.device_power.on")]
	MachineDevicePowerOn,
	#[serde(rename = "machine.device_power.off")]
	MachineDevicePowerOff,
	#[serde(rename = "machine.wled.strips")]
	MachineWledStrips,
	#[serde(rename = "machine.wled.status")]
	MachineWledStatus,
	#[serde(rename = "machine.wled.on")]
	MachineWledOn,
	#[serde(rename = "machine.wled.off")]
	MachineWledOff,
	#[serde(rename = "machine.wled.toggle")]
	MachineWledToggle,
	#[serde(rename = "machine.wled.get_strip")]
	MachineWledGetStrip,
	#[serde(rename = "server.history.list")]
	ServerHistoryList,
	#[serde(rename = "server.history.totals")]
	ServerHistoryTotals,
	#[serde(rename = "server.history.reset_totals")]
	ServerHistoryResetTotals,
	#[serde(rename = "server.history.get_job")]
	ServerHistoryGetJob,
	#[serde(rename = "server.history.delete_job")]
	ServerHistoryDeleteJob,
	#[serde(rename = "server.mqtt.publish")]
	ServerMgttPublish,
	#[serde(rename = "server.mqtt.subscribe")]
	ServerMqttSubscribe,
	#[serde(rename = "server.extensions.list")]
	ServerExtensionList,
	#[serde(rename = "server.extensions.request")]
	ServerExtensionsRequest,
	#[serde(rename = "connection.send_event")]
	ConnectionSendEvent,
	#[serde(untagged)]
	Other(String),
}

