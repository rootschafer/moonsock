// use moonsock::{
//     MoonResponse, JsonRpcVersion,
//     response::{MoonResultData, Server, FileManager, Machine, JobQueue, Authorization, Mqtt, Config, Orig, File, ServerConfig},
// };

// #[test]
// fn test_deserialize_server_config() {
//     // let json = r#"{
//     //     "jsonrpc": "2.0",
//     //     "result": {
//     //         "config": {
//     //             "server": {
//     //                 "host": "0.0.0.0",
//     //                 "port": 7125,
//     //                 "ssl_port": 7130,
//     //                 "enable_debug_logging": true,
//     //                 "enable_asyncio_debug": false,
//     //                 "klippy_uds_address": "/tmp/klippy_uds",
//     //                 "max_upload_size": 210,
//     //                 "ssl_certificate_path": null,
//     //                 "ssl_key_path": null
//     //             },
//     //             "dbus_manager": {},
//     //             "database": {},
//     //             "file_manager": {
//     //                 "enable_object_processing": true,
//     //                 "queue_gcode_uploads": true,
//     //                 "config_path": "~/printer_config",
//     //                 "log_path": "~/logs"
//     //             },
//     //             // ... other fields ...
//     //         },
//     //         "orig": {
//     //             "DEFAULT": {},
//     //             "server": {
//     //                 "enable_debug_logging": "True",
//     //                 "max_upload_size": "210"
//     //             },
//     //             "file_manager": {
//     //                 "config_path": "~/printer_config",
//     //                 "log_path": "~/logs",
//     //                 "queue_gcode_uploads": "True",
//     //                 "enable_object_processing": "True"
//     //             },
//     //             // ... other fields ...
//     //         },
//     //         "files": [
//     //             {
//     //                 "filename": "moonraker.conf",
//     //                 "sections": [
//     //                     "server",
//     //                     "file_manager",
//     //                     "machine",
//     //                     "announcements",
//     //                     "job_queue",
//     //                     "authorization",
//     //                     "zeroconf",
//     //                     "octoprint_compat",
//     //                     "history",
//     //                     "secrets"
//     //                 ]
//     //             },
//     //             {
//     //                 "filename": "include/extras.conf",
//     //                 "sections": [
//     //                     "mqtt"
//     //                 ]
//     //             }
//     //         ]
//     //     },
//     //     "id": 354
//     // }"#;
//     // let empty_json_object = "{}";
//     // let empty_value: serde_json::Value = serde_json::from_str(&empty_json_object).unwrap();
//     // println!("Deserialized Empty Value:\n{empty_value:?}");

//     let json = r##"{
//         "jsonrpc": "2.0",
//         "result": {
//             "config": {
//                 "server": {
//                     "host": "0.0.0.0",
//                     "port": 7125,
//                     "ssl_port": 7130,
//                     "enable_debug_logging": true,
//                     "enable_asyncio_debug": false,
//                     "klippy_uds_address": "/tmp/klippy_uds",
//                     "max_upload_size": 210,
//                     "ssl_certificate_path": null,
//                     "ssl_key_path": null
//                 },
//                 "dbus_manager": {},
//                 "database": {
//                     "database_path": "~/.moonraker_database",
//                     "enable_database_debug": false
//                 },
//                 "file_manager": {
//                     "enable_object_processing": true,
//                     "queue_gcode_uploads": true,
//                     "config_path": "~/printer_config",
//                     "log_path": "~/logs"
//                 },
//                 "klippy_apis": {},
//                 "machine": {
//                     "provider": "systemd_dbus"
//                 },
//                 "shell_command": {},
//                 "data_store": {
//                     "temperature_store_size": 1200,
//                     "gcode_store_size": 1000
//                 },
//                 "proc_stats": {},
//                 "job_state": {},
//                 "job_queue": {
//                     "load_on_startup": true,
//                     "automatic_transition": false,
//                     "job_transition_delay": 2,
//                     "job_transition_gcode": "\nM118 Transitioning to next job..."
//                 },
//                 "http_client": {},
//                 "announcements": {
//                     "dev_mode": false,
//                     "subscriptions": []
//                 },
//                 "authorization": {
//                     "login_timeout": 90,
//                     "force_logins": false,
//                     "cors_domains": [
//                         "*.home",
//                         "http://my.mainsail.xyz",
//                         "http://app.fluidd.xyz",
//                         "*://localhost:*"
//                     ],
//                     "trusted_clients": [
//                         "192.168.1.0/24"
//                     ]
//                 },
//                 "zeroconf": {},
//                 "octoprint_compat": {
//                     "enable_ufp": true,
//                     "flip_h": false,
//                     "flip_v": false,
//                     "rotate_90": false,
//                     "stream_url": "/webcam/?action=stream",
//                     "webcam_enabled": true
//                 },
//                 "history": {},
//                 "secrets": {
//                     "secrets_path": "~/moonraker_secrets.ini"
//                 },
//                 "mqtt": {
//                     "address": "eric-work.home",
//                     "port": 1883,
//                     "username": "username",
//                     "password_file": null,
//                     "password": "password",
//                     "mqtt_protocol": "v3.1.1",
//                     "instance_name": "pi-debugger",
//                     "default_qos": 0,
//                     "status_objects": {
//                         "webhooks": null,
//                         "toolhead": "position,print_time",
//                         "idle_timeout": "state",
//                         "gcode_macro M118": null
//                     },
//                     "api_qos": 0,
//                     "enable_moonraker_api": true
//                 },
//                 "template": {}
//             },
//             "orig": {
//                 "DEFAULT": {},
//                 "server": {
//                     "enable_debug_logging": "True",
//                     "max_upload_size": "210"
//                 },
//                 "file_manager": {
//                     "config_path": "~/printer_config",
//                     "log_path": "~/logs",
//                     "queue_gcode_uploads": "True",
//                     "enable_object_processing": "True"
//                 },
//                 "machine": {
//                     "provider": "systemd_dbus"
//                 },
//                 "announcements": {},
//                 "job_queue": {
//                     "job_transition_delay": "2.",
//                     "job_transition_gcode": "\nM118 Transitioning to next job...",
//                     "load_on_startup": "True"
//                 },
//                 "authorization": {
//                     "trusted_clients": "\n192.168.1.0/24",
//                     "cors_domains": "\n*.home\nhttp://my.mainsail.xyz\nhttp://app.fluidd.xyz\n*://localhost:*"
//                 },
//                 "zeroconf": {},
//                 "octoprint_compat": {},
//                 "history": {},
//                 "secrets": {
//                     "secrets_path": "~/moonraker_secrets.ini"
//                 },
//                 "mqtt": {
//                     "address": "eric-work.home",
//                     "port": "1883",
//                     "username": "{secrets.mqtt_credentials.username}",
//                     "password": "{secrets.mqtt_credentials.password}",
//                     "enable_moonraker_api": "True",
//                     "status_objects": "\nwebhooks\ntoolhead=position,print_time\nidle_timeout=state\ngcode_macro M118"
//                 }
//             },
//             "files": [
//                 {
//                     "filename": "moonraker.conf",
//                     "sections": [
//                         "server",
//                         "file_manager",
//                         "machine",
//                         "announcements",
//                         "job_queue",
//                         "authorization",
//                         "zeroconf",
//                         "octoprint_compat",
//                         "history",
//                         "secrets"
//                     ]
//                 },
//                 {
//                     "filename": "include/extras.conf",
//                     "sections": [
//                         "mqtt"
//                     ]
//                 }
//             ]
//         },
//         "id": 354
//     }"##;

//     let response: MoonResponse = serde_json::from_str(json).unwrap();

//     match response {
//         MoonResponse::MoonResult { result, .. } => match result {
//             MoonResultData::ServerConfig(_) => (),
//             _ => panic!("Expected ServerConfig"),
//         },
//         _ => panic!("Expected MoonResult"),
//     }
// }

// #[test]
// fn test_serialize_server_config() {
//     let server = Server {
//         host: "0.0.0.0".to_string(),
//         port: 7125,
//         ssl_port: 7130,
//         enable_debug_logging: true,
//         enable_asyncio_debug: false,
//         klippy_uds_address: "/tmp/klippy_uds".to_string(),
//         max_upload_size: 210,
//         ssl_certificate_path: None,
//         ssl_key_path: None,
//     };

//     let file_manager = FileManager {
//         enable_object_processing: true,
//         queue_gcode_uploads: true,
//         config_path: "~/printer_config".to_string(),
//         log_path: "~/logs".to_string(),
//     };

//     let machine = Machine {
//         provider: "systemd_dbus".to_string(),
//     };

//     let job_queue = JobQueue {
//         load_on_startup: true,
//         automatic_transition: false,
//         job_transition_delay: 2,
//         job_transition_gcode: "\nM118 Transitioning to next job...".to_string(),
//     };

//     let authorization = Authorization {
//         login_timeout: 90,
//         force_logins: false,
//         cors_domains: vec![
//             "*.home".to_string(),
//             "http://my.mainsail.xyz".to_string(),
//             "http://app.fluidd.xyz".to_string(),
//             "*://localhost:*".to_string(),
//         ],
//         trusted_clients: vec!["192.168.1.0/24".to_string()],
//     };

//     let mqtt = Mqtt {
//         address: "eric-work.home".to_string(),
//         port: 1883,
//         username: "{secrets.mqtt_credentials.username}".to_string(),
//         password_file: None,
//         password: "{secrets.mqtt_credentials.password}".to_string(),
//         mqtt_protocol: "v3.1.1".to_string(),
//         instance_name: "pi-debugger".to_string(),
//         default_qos: 0,
//         status_objects: serde_json::json!({
//             "webhooks": null,
//             "toolhead": "position,print_time",
//             "idle_timeout": "state",
//             "gcode_macro M118": null
//         }),
//         api_qos: 0,
//         enable_moonraker_api: true,
//     };

//     let config = Config {
//         server: server.clone(),
//         dbus_manager: serde_json::Value::Object(serde_json::Map::new()),
//         database: serde_json::Value::Object(serde_json::Map::new()),
//         file_manager: file_manager.clone(),
//         klippy_apis: serde_json::Value::Object(serde_json::Map::new()),
//         machine: machine.clone(),
//         shell_command: serde_json::Value::Object(serde_json::Map::new()),
//         data_store: serde_json::Value::Object(serde_json::Map::new()),
//         proc_stats: serde_json::Value::Object(serde_json::Map::new()),
//         job_state: serde_json::Value::Object(serde_json::Map::new()),
//         job_queue: job_queue.clone(),
//         http_client: serde_json::Value::Object(serde_json::Map::new()),
//         announcements: serde_json::Value::Object(serde_json::Map::new()),
//         authorization: authorization.clone(),
//         zeroconf: serde_json::Value::Object(serde_json::Map::new()),
//         octoprint_compat: serde_json::Value::Object(serde_json::Map::new()),
//         history: serde_json::Value::Object(serde_json::Map::new()),
//         secrets: serde_json::Value::Object(serde_json::Map::new()),
//         mqtt: mqtt.clone(),
//         template: serde_json::Value::Object(serde_json::Map::new()),
//     };

//     let orig = Orig {
//         // default: serde_json::Value::Object(serde_json::Map::new()),
//         default: serde_json::json!({}),
//         // server,
//         server: serde_json::json!({
//             "enable_debug_logging": "True",
//             "max_upload_size": "210"
//         }),
//         // file_manager,
//         file_manager: serde_json::json!({
//             "config_path": "~/printer_config",
//             "log_path": "~/logs",
//             "queue_gcode_uploads": "True",
//             "enable_object_processing": "True"
//         }),
//         machine,
//         // announcements: serde_json::Value::Object(serde_json::Map::new()),
//         announcements: serde_json::json!({}),
//         // job_queue,
//         job_queue: serde_json::json!({
//             "job_transition_delay": "2.",
//             "job_transition_gcode": "\nM118 Transitioning to next job...",
//             "load_on_startup": "True"
//         }),
//         // authorization,
//         authorization: serde_json::json!({
//             "trusted_clients": "\n192.168.1.0/24",
//             "cors_domains": "\n*.home\nhttp://my.mainsail.xyz\nhttp://app.fluidd.xyz\n*://localhost:*"
//         }),
//         // zeroconf: serde_json::Value::Object(serde_json::Map::new()),
//         zeroconf: serde_json::json!({}),
//         // octoprint_compat: serde_json::Value::Object(serde_json::Map::new()),
//         octoprint_compat: serde_json::json!({}),
//         // history: serde_json::Value::Object(serde_json::Map::new()),
//         history: serde_json::json!({}),
//         // secrets: serde_json::Value::Object(serde_json::Map::new()),
//         secrets: serde_json::json!({
//             "secrets_path": "~/moonraker_secrets.ini"
//         }),
//         // mqtt,
//         mqtt: serde_json::json!({
//             "address": "eric-work.home",
//             "port": "1883",
//             "username": "{secrets.mqtt_credentials.username}",
//             "password": "{secrets.mqtt_credentials.password}",
//             "enable_moonraker_api": "True",
//             "status_objects": "\nwebhooks\ntoolhead=position,print_time\nidle_timeout=state\ngcode_macro M118"
//         }),
//     };

//     let files = vec![
//         File {
//             filename: "moonraker.conf".to_string(),
//             sections: vec![
//                 "server".to_string(),
//                 "file_manager".to_string(),
//                 "machine".to_string(),
//                 "announcements".to_string(),
//                 "job_queue".to_string(),
//                 "authorization".to_string(),
//                 "zeroconf".to_string(),
//                 "octoprint_compat".to_string(),
//                 "history".to_string(),
//                 "secrets".to_string(),
//             ],
//         },
//         File {
//             filename: "include/extras.conf".to_string(),
//             sections: vec!["mqtt".to_string()],
//         },
//     ];

//     let server_config = ServerConfig {
//         config,
//         orig,
//         files,
//     };

//     let response = MoonResponse::default_server_config_result(server_config);

//     let json = serde_json::to_string(&response).unwrap();

//     assert!(json.contains(r#""jsonrpc":"2.0""#));
//     assert!(json.contains(r#""server":{"host":"0.0.0.0","port":7125,"ssl_port":7130,"enable_debug_logging":true,"enable_asyncio_debug":false,"klippy_uds_address":"/tmp/klippy_uds","max_upload_size":210,"ssl_certificate_path":null,"ssl_key_path":null}"#));
//     // ... assert other fields ...
// }

// #[test]
// fn test_deserialize_server_config() {
//     let json = r#"{
//         "jsonrpc": "2.0",
//         "result": {
//             "config": {
//                 "server": {
//                     "host": "0.0.0.0",
//                     "port": 7125,
//                     "ssl_port": 7130,
//                     "enable_debug_logging": true,
//                     "enable_asyncio_debug": false,
//                     "klippy_uds_address": "/tmp/klippy_uds",
//                     "max_upload_size": 210,
//                     "ssl_certificate_path": null,
//                     "ssl_key_path": null
//                 },
//                 "dbus_manager": {},
//                 "database": {},
//                 "file_manager": {
//                     "enable_object_processing": true,
//                     "queue_gcode_uploads": true,
//                     "config_path": "~/printer_config",
//                     "log_path": "~/logs"
//                 },
//                 "klippy_apis": {},
//                 "machine": {
//                     "provider": "systemd_dbus"
//                 },
//                 "shell_command": {},
//                 "data_store": {},
//                 "proc_stats": {},
//                 "job_state": {},
//                 "job_queue": {
//                     "load_on_startup": true,
//                     "automatic_transition": false,
//                     "job_transition_delay": 2,
//                     "job_transition_gcode": "\nM118 Transitioning to next job..."
//                 },
//                 "http_client": {},
//                 "announcements": {},
//                 "authorization": {
//                     "login_timeout": 90,
//                     "force_logins": false,
//                     "cors_domains": [
//                         "*.home",
//                         "http://my.mainsail.xyz",
//                         "http://app.fluidd.xyz",
//                         "*://localhost:*"
//                     ],
//                     "trusted_clients": [
//                         "192.168.1.0/24"
//                     ]
//                 },
//                 "zeroconf": {},
//                 "octoprint_compat": {},
//                 "history": {},
//                 "secrets": {
//                     "secrets_path": "~/moonraker_secrets.ini"
//                 },
//                 "mqtt": {
//                     "address": "eric-work.home",
//                     "port": 1883,
//                     "username": "{secrets.mqtt_credentials.username}",
//                     "password_file": null,
//                     "password": "{secrets.mqtt_credentials.password}",
//                     "mqtt_protocol": "v3.1.1",
//                     "instance_name": "pi-debugger",
//                     "default_qos": 0,
//                     "status_objects": {
//                         "webhooks": null,
//                         "toolhead": "position,print_time",
//                         "idle_timeout": "state",
//                         "gcode_macro M118": null
//                     },
//                     "api_qos": 0,
//                     "enable_moonraker_api": true
//                 },
//                 "template": {}
//             },
//             "orig": {
//                 "DEFAULT": {},
//                 "server": {
//                     "enable_debug_logging": "True",
//                     "max_upload_size": "210"
//                 },
//                 "file_manager": {
//                     "config_path": "~/printer_config",
//                     "log_path": "~/logs",
//                     "queue_gcode_uploads": "True",
//                     "enable_object_processing": "True"
//                 },
//                 "machine": {
//                     "provider": "systemd_dbus"
//                 },
//                 "announcements": {},
//                 "job_queue": {
//                     "job_transition_delay": "2.",
//                     "job_transition_gcode": "\nM118 Transitioning to next job...",
//                     "load_on_startup": "True"
//                 },
//                 "authorization": {
//                     "trusted_clients": "\n192.168.1.0/24",
//                     "cors_domains": "\n*.home\nhttp://my.mainsail.xyz\nhttp://app.fluidd.xyz\n*://localhost:*"
//                 },
//                 "zeroconf": {},
//                 "octoprint_compat": {},
//                 "history": {},
//                 "secrets": {
//                     "secrets_path": "~/moonraker_secrets.ini"
//                 },
//                 "mqtt": {
//                     "address": "eric-work.home",
//                     "port": "1883",
//                     "username": "{secrets.mqtt_credentials.username}",
//                     "password": "{secrets.mqtt_credentials.password}",
//                     "enable_moonraker_api": "True",
//                     "status_objects": "\nwebhooks\ntoolhead=position,print_time\nidle_timeout=state\ngcode_macro M118"
//                 }
//             },
//             "files": [
//                 {
//                     "filename": "moonraker.conf",
//                     "sections": [
//                         "server",
//                         "file_manager",
//                         "machine",
//                         "announcements",
//                         "job_queue",
//                         "authorization",
//                         "zeroconf",
//                         "octoprint_compat",
//                         "history",
//                         "secrets"
//                     ]
//                 },
//                 {
//                     "filename": "include/extras.conf",
//                     "sections": [
//                         "mqtt"
//                     ]
//                 }
//             ]
//         },
//         "id": 354
//     }"#;

//     let response: MoonResponse = serde_json::from_str(json).unwrap();

//     match response {
//         MoonResponse::MoonResult { result, .. } => match result {
//             MoonResultData::ServerConfig(_) => (),
//             _ => panic!("Expected ServerConfig"),
//         },
//         _ => panic!("Expected MoonResult"),
//     }
// }

// #[test]
// fn test_serialize_server_config() {
//     let server = Server {
//         host: "0.0.0.0".to_string(),
//         port: 7125,
//         ssl_port: 7130,
//         enable_debug_logging: true,
//         enable_asyncio_debug: false,
//         klippy_uds_address: "/tmp/klippy_uds".to_string(),
//         max_upload_size: 210,
//         ssl_certificate_path: None,
//         ssl_key_path: None,
//     };

//     let file_manager = FileManager {
//         enable_object_processing: true,
//         queue_gcode_uploads: true,
//         config_path: "~/printer_config".to_string(),
//         log_path: "~/logs".to_string(),
//     };

//     let machine = Machine {
//         provider: "systemd_dbus".to_string(),
//     };

//     let job_queue = JobQueue {
//         load_on_startup: true,
//         automatic_transition: false,
//         job_transition_delay: 2,
//         job_transition_gcode: "\nM118 Transitioning to next job...".to_string(),
//     };

//     let authorization = Authorization {
//         login_timeout: 90,
//         force_logins: false,
//         cors_domains: vec![
//             "*.home".to_string(),
//             "http://my.mainsail.xyz".to_string(),
//             "http://app.fluidd.xyz".to_string(),
//             "*://localhost:*".to_string(),
//         ],
//         trusted_clients: vec!["192.168.1.0/24".to_string()],
//     };

//     let mqtt = Mqtt {
//         address: "eric-work.home".to_string(),
//         port: 1883,
//         username: "{secrets.mqtt_credentials.username}".to_string(),
//         password_file: None,
//         password: "{secrets.mqtt_credentials.password}".to_string(),
//         mqtt_protocol: "v3.1.1".to_string(),
//         instance_name: "pi-debugger".to_string(),
//         default_qos: 0,
//         status_objects: serde_json::json!({
//             "webhooks": null,
//             "toolhead": "position,print_time",
//             "idle_timeout": "state",
//             "gcode_macro M118": null
//         }),
//         api_qos: 0,
//         enable_moonraker_api: true,
//     };

//     let config = Config {
//         server,
//         dbus_manager: serde_json::json!({}),
//         database: serde_json::json!({}),
//         file_manager,
//         klippy_apis: serde_json::json!({}),
//         machine,
//         shell_command: serde_json::json!({}),
//         data_store: serde_json::json!({}),
//         proc_stats: serde_json::json!({}),
//         job_state: serde_json::json!({}),
//         job_queue,
//         http_client: serde_json::json!({}),
//         announcements: serde_json::json!({}),
//         authorization,
//         zeroconf: serde_json::json!({}),
//         octoprint_compat: serde_json::json!({}),
//         history: serde_json::json!({}),
//         secrets: serde_json::json!({
//             "secrets_path": "~/moonraker_secrets.ini"
//         }),
//         mqtt,
//         template: serde_json::json!({}),
//     };

//     let orig = Orig {
//         DEFAULT: serde_json::json!({}),
//         server: Server {
//             enable_debug_logging: true,
//             max_upload_size: 210,
//             ..Default::default()
//         },
//         file_manager: FileManager {
//             config_path: "~/printer_config".to_string(),
//             log_path: "~/logs".to_string(),
//             queue_gcode_uploads: true,
//             enable_object_processing: true,
//         },
//         machine: Machine {
//             provider: "systemd_dbus".to_string(),
//         },
//         announcements: serde_json::json!({}),
//         job_queue: JobQueue {
//             job_transition_delay: 2,
//             job_transition_gcode: "\nM118 Transitioning to next job...".to_string(),
//             load_on_startup: true,
//             ..Default::default()
//         },
//         authorization: Authorization {
//             trusted_clients: vec!["192.168.1.0/24".to_string()],
//             cors_domains: vec![
//                 "*.home".to_string(),
//                 "http://my.mainsail.xyz".to_string(),
//                 "http://app.fluidd.xyz".to_string(),
//                 "*://localhost:*".to_string(),
//             ],
//             ..Default::default()
//         },
//         zeroconf: serde_json::json!({}),
//         octoprint_compat: serde_json::json!({}),
//         history: serde_json::json!({}),
//         secrets: serde_json::json!({
//             "secrets_path": "~/moonraker_secrets.ini"
//         }),
//         mqtt: Mqtt {
//             address: "eric-work.home".to_string(),
//             port: 1883,
//             username: "{secrets.mqtt_credentials.username}".to_string(),
//             password: "{secrets.mqtt_credentials.password}".to_string(),
//             enable_moonraker_api: true,
//             status_objects: serde_json::json!("\nwebhooks\ntoolhead=position,print_time\nidle_timeout=state\ngcode_macro M118"),
//             ..Default::default()
//         },
//     };

//     let files = vec![
//         File {
//             filename: "moonraker.conf".to_string(),
//             sections: vec![
//                 "server".to_string(),
//                 "file_manager".to_string(),
//                 "machine".to_string(),
//                 "announcements".to_string(),
//                 "job_queue".to_string(),
//                 "authorization".to_string(),
//                 "zeroconf".to_string(),
//                 "octoprint_compat".to_string(),
//                 "history".to_string(),
//                 "secrets".to_string(),
//             ],
//         },
//         File {
//             filename: "include/extras.conf".to_string(),
//             sections: vec!["mqtt".to_string()],
//         },
//     ];

//     let server_config = ServerConfig {
//         config,
//         orig,
//         files,
//     };

//     let response = MoonResponse::MoonResult {
//         jsonrpc: JsonRpcVersion::V2,
//         result: MoonResultData::ServerConfig(server_config),
//         id: 354,
//     };

//     let json = serde_json::to_string(&response).unwrap();

//     assert!(json.contains(r#""jsonrpc":"2.0""#));
//     assert!(json.contains(r#""id":354"#));
//     // ... check other fields ...
// }
