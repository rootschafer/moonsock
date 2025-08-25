use maplit::hashmap;
use moonsock::MoonNotification;
use moonsock::{
	ApplicationInfo,
	CommitInfo,
	GitRepoInfo,
	JsonRpcVersion,
	MoonResponse,
	NotificationMethod,
	NotificationParam,
	// response::SystemInfo,
	SystemPkgInfo,
	UpdateRefreshedParam,
	VersionInfo,
	WebInfo,
};

#[test]
fn test_parse_notify_update_refreshed() {
	let json = r#"{
        "jsonrpc": "2.0",
        "method": "notify_update_refreshed",
        "params": [
            {
                "busy": false,
                "github_rate_limit": 60,
                "github_requests_remaining": 57,
                "github_limit_reset_time": 1615836932,
                "version_info": {
                    "system": {
                        "package_count": 4,
                        "package_list": [
                            "libtiff5",
                            "raspberrypi-sys-mods",
                            "rpi-eeprom-images",
                            "rpi-eeprom"
                        ]
                    },
                    "moonraker": {
                        "channel": "dev",
                        "debug_enabled": true,
                        "is_valid": true,
                        "configured_type": "git_repo",
                        "corrupt": false,
                        "info_tags": [],
                        "detected_type": "git_repo",
                        "remote_alias": "arksine",
                        "branch": "master",
                        "owner": "?",
                        "repo_name": "moonraker",
                        "version": "v0.7.1-364",
                        "remote_version": "v0.7.1-364",
                        "rollback_version": "v0.7.1-360",
                        "current_hash": "ecfad5cff15fff1d82cb9bdc64d6b548ed53dfaf",
                        "remote_hash": "ecfad5cff15fff1d82cb9bdc64d6b548ed53dfaf",
                        "is_dirty": false,
                        "detached": true,
                        "commits_behind": [],
                        "git_messages": [],
                        "full_version_string": "v0.7.1-364-gecfad5c",
                        "pristine": true,
                        "recovery_url": "https://github.com/Arksine/moonraker.git",
                        "remote_url": "https://github.com/Arksine/moonraker.git",
                        "warnings": [],
                        "anomalies": [
                            "Unofficial remote url: https://github.com/Arksine/moonraker-fork.git",
                            "Repo not on offical remote/branch, expected: origin/master, detected: altremote/altbranch",
                            "Detached HEAD detected"
                        ]
                    },
                    "mainsail": {
                        "name": "mainsail",
                        "owner": "mainsail-crew",
                        "version": "v2.1.1",
                        "remote_version": "v2.1.1",
                        "rollback_version": "v2.0.0",
                        "configured_type": "web",
                        "channel": "stable",
                        "info_tags": [
                            "desc=Mainsail Web Client",
                            "action=some_action"
                        ],
                        "warnings": [],
                        "anomalies": [],
                        "is_valid": true
                    },
                    "fluidd": {
                        "name": "fluidd",
                        "owner": "fluidd-core",
                        "version": "v1.16.2",
                        "remote_version": "v1.16.2",
                        "rollback_version": "v1.15.0",
                        "configured_type": "web",
                        "channel": "beta",
                        "info_tags": [],
                        "warnings": [],
                        "anomalies": [],
                        "is_valid": true
                    },
                    "klipper": {
                        "channel": "dev",
                        "debug_enabled": true,
                        "is_valid": true,
                        "configured_type": "git_repo",
                        "corrupt": false,
                        "info_tags": [],
                        "detected_type": "git_repo",
                        "remote_alias": "origin",
                        "branch": "master",
                        "owner": "Klipper3d",
                        "repo_name": "klipper",
                        "version": "v0.10.0-1",
                        "remote_version": "v0.10.0-41",
                        "rollback_version": "v0.9.1-340",
                        "current_hash": "4c8d24ae03eadf3fc5a28efb1209ce810251d02d",
                        "remote_hash": "e3cbe7ea3663a8cd10207a9aecc4e5458aeb1f1f",
                        "is_dirty": false,
                        "detached": false,
                        "commits_behind": [
                            {
                                "sha": "e3cbe7ea3663a8cd10207a9aecc4e5458aeb1f1f",
                                "author": "Kevin O'Connor",
                                "date": "1644534721",
                                "subject": "stm32: Clear SPE flag on a change to SPI CR1 register",
                                "message": "The stm32 specs indicate that the SPE bit must be cleared before\nchanging the CPHA or CPOL bits.\n\nReported by @cbc02009 and @bigtreetech.\n\nSigned-off-by: Kevin O'Connor ",
                                "tag": null
                            },
                            {
                                "sha": "99d55185a21703611b862f6ce4b80bba70a9c4b5",
                                "author": "Kevin O'Connor",
                                "date": "1644532075",
                                "subject": "stm32: Wait for transmission to complete before returning from spi_transfer()",
                                "message": "It's possible for the SCLK pin to still be updating even after the\nlast byte of data has been read from the receive pin.  (In particular\nin spi mode 0 and 1.)  Exiting early from spi_transfer() in this case\ncould result in the CS pin being raised before the final updates to\nSCLK pin.\n\nAdd an additional wait at the end of spi_transfer() to avoid this\nissue.\n\nSigned-off-by: Kevin O'Connor ",
                                "tag": null
                            }
                        ],
                        "git_messages": [],
                        "full_version_string": "v0.10.0-1-g4c8d24ae-shallow",
                        "pristine": true,
                        "recovery_url": "https://github.com/Klipper3d/klipper.git",
                        "remote_url": "https://github.com/Klipper3d/klipper.git",
                        "warnings": [],
                        "anomalies": []
                    }
                }
            }
        ]
    }"#;

	let expected = MoonNotification {
		jsonrpc: JsonRpcVersion::V2,
		method: NotificationMethod::NotifyUpdateRefreshed,
		// params: Some(vec![
		params: Some(NotificationParam::UpdateRefreshed(vec![UpdateRefreshedParam {
			busy: false,
			github_rate_limit: 60,
			github_requests_remaining: 57,
			github_limit_reset_time: 1615836932,
			version_info: VersionInfo {
				system: SystemPkgInfo {
					package_count: 4,
					package_list: vec![
						"libtiff5".to_string(),
						"raspberrypi-sys-mods".to_string(),
						"rpi-eeprom-images".to_string(),
						"rpi-eeprom".to_string(),
					],
				},
				applications: hashmap! {
					   "moonraker".to_string() => ApplicationInfo::GitRepo(GitRepoInfo {
						   channel: "dev".to_string(),
						   debug_enabled: true,
						   is_valid: true,
						   configured_type: "git_repo".to_string(),
						   corrupt: false,
						   info_tags: vec![],
						   detected_type: "git_repo".to_string(),
						   remote_alias: "arksine".to_string(),
						   branch: "master".to_string(),
						   owner: "?".to_string(),
						   repo_name: "moonraker".to_string(),
						   version: "v0.7.1-364".to_string(),
						   remote_version: "v0.7.1-364".to_string(),
						   rollback_version: "v0.7.1-360".to_string(),
						   current_hash: "ecfad5cff15fff1d82cb9bdc64d6b548ed53dfaf".to_string(),
						   remote_hash: "ecfad5cff15fff1d82cb9bdc64d6b548ed53dfaf".to_string(),
						   is_dirty: false,
						   detached: true,
						   commits_behind: vec![],
						   git_messages: vec![],
						   full_version_string: "v0.7.1-364-gecfad5c".to_string(),
						   pristine: true,
						   recovery_url: "https://github.com/Arksine/moonraker.git".to_string(),
						   remote_url: "https://github.com/Arksine/moonraker.git".to_string(),
						   warnings: vec![],
						   anomalies: vec![
							   "Unofficial remote url: https://github.com/Arksine/moonraker-fork.git".to_string(),
							   "Repo not on offical remote/branch, expected: origin/master, detected: altremote/altbranch".to_string(),
							   "Detached HEAD detected".to_string(),
						   ],
					   }),
					   "mainsail".to_string() => ApplicationInfo::Web(WebInfo {
						   name: "mainsail".to_string(),
						   owner: "mainsail-crew".to_string(),
						   version: "v2.1.1".to_string(),
						   remote_version: "v2.1.1".to_string(),
						   rollback_version: "v2.0.0".to_string(),
						   configured_type: "web".to_string(),
						   channel: "stable".to_string(),
						   info_tags: vec![
							   "desc=Mainsail Web Client".to_string(),
							   "action=some_action".to_string(),
						   ],
						   warnings: vec![],
						   anomalies: vec![],
						   is_valid: true,
					   }),
					   "fluidd".to_string() => ApplicationInfo::Web(WebInfo {
						   name: "fluidd".to_string(),
				owner: "fluidd-core".to_string(),
						   version: "v1.16.2".to_string(),
						   remote_version: "v1.16.2".to_string(),
						   rollback_version: "v1.15.0".to_string(),
						   configured_type: "web".to_string(),
						   channel: "beta".to_string(),
						   info_tags: vec![],
						   warnings: vec![],
						   anomalies: vec![],
						   is_valid: true,
					   }),
					   "klipper".to_string() => ApplicationInfo::GitRepo(GitRepoInfo {
						   channel: "dev".to_string(),
						   debug_enabled: true,
						   is_valid: true,
						   configured_type: "git_repo".to_string(),
						   corrupt: false,
						   info_tags: vec![],
						   detected_type: "git_repo".to_string(),
						   remote_alias: "origin".to_string(),
						   branch: "master".to_string(),
						   owner: "Klipper3d".to_string(),
						   repo_name: "klipper".to_string(),
						   version: "v0.10.0-1".to_string(),
						   remote_version: "v0.10.0-41".to_string(),
						   rollback_version: "v0.9.1-340".to_string(),
						   current_hash: "4c8d24ae03eadf3fc5a28efb1209ce810251d02d".to_string(),
						   remote_hash: "e3cbe7ea3663a8cd10207a9aecc4e5458aeb1f1f".to_string(),
						   is_dirty: false,
						   detached: false,
						   commits_behind: vec![
							   CommitInfo {
								   sha: "e3cbe7ea3663a8cd10207a9aecc4e5458aeb1f1f".to_string(),
								   author: "Kevin O'Connor".to_string(),
								   // date: 1644534721,
								   date: "1644534721".to_string(),
								   subject: "stm32: Clear SPE flag on a change to SPI CR1 register".to_string(),
								   message: "The stm32 specs indicate that the SPE bit must be cleared before\nchanging the CPHA or CPOL bits.\n\nReported by @cbc02009 and @bigtreetech.\n\nSigned-off-by: Kevin O'Connor ".to_string(),
								   tag: None,
							   },
							   CommitInfo {
								   sha: "99d55185a21703611b862f6ce4b80bba70a9c4b5".to_string(),
								   author: "Kevin O'Connor".to_string(),
								   // date: 1644532075,
								   date: "1644532075".to_string(),
								   subject: "stm32: Wait for transmission to complete before returning from spi_transfer()".to_string(),
								   message: "It's possible for the SCLK pin to still be updating even after the\nlast byte of data has been read from the receive pin.  (In particular\nin spi mode 0 and 1.)  Exiting early from spi_transfer() in this case\ncould result in the CS pin being raised before the final updates to\nSCLK pin.\n\nAdd an additional wait at the end of spi_transfer() to avoid this\nissue.\n\nSigned-off-by: Kevin O'Connor ".to_string(),
								   tag: None,
							   },
						   ],
						   git_messages: vec![],
						   full_version_string: "v0.10.0-1-g4c8d24ae-shallow".to_string(),
						   pristine: true,
						   recovery_url: "https://github.com/Klipper3d/klipper.git".to_string(),
						   remote_url: "https://github.com/Klipper3d/klipper.git".to_string(),
						   warnings: vec![],
						   anomalies: vec![],
					   }),
				   },
			},
		}])),
	};

	// let actual: MoonResponse = serde_json::from_str(json).unwrap();
	let actual: MoonNotification = serde_json::from_str(json).unwrap();
	assert_eq!(actual, expected);

	let serialized = serde_json::to_string(&actual).unwrap();
	let deserialized: MoonNotification = serde_json::from_str(&serialized).unwrap();
	assert_eq!(deserialized, expected);
}

