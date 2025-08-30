// use serde_json
// use moonsock::{
//     MoonResponse, JsonRpcVersion,
//     NotificationMethod, NotificationParam,
//     FilelistChangedParam, FilelistAction, FilelistItem,
// };

// #[test]
// fn test_parse_notify_filelist_changed() {
//     let json = r#"{
//         "jsonrpc": "2.0",
//         "method": "notify_filelist_changed",
//         "params": [
//             {
//                 "action": "create_file",
//                 "item": {
//                     "path": "some/path",
//                     "root": "root_name",
//                     "size": 46458,
//                     "modified": 545465,
//                     "permissions": "rw"
//                 },
//                 "source_item": {
//                     "path": "some/source/path",
//                     "root": "root_name"
//                 }
//             }
//         ]
//     }"#;

//     let expected = MoonNotification {
//         jsonrpc: JsonRpcVersion::V2,
//         method: NotificationMethod::NotifyFilelistChanged,
//         params: Some(
//             NotificationParam::FilelistChanged(vec![FilelistChangedParam {
//                 action: FilelistAction::CreateFile,
//                 item: FilelistItem {
//                     path: "some/path".to_string(),
//                     root: "root_name".to_string(),
//                     size: Some(46458),
//                     modified: Some(545465),
//                     permissions: Some("rw".to_string()),
//                 },
//                 source_item: Some(FilelistItem {
//                     path: "some/source/path".to_string(),
//                     root: "root_name".to_string(),
//                     size: None,
//                     modified: None,
//                     permissions: None,
//                 }),
//             }]),
//         ),
//     };

//     let actual: MoonResponse = serde_json::from_str(json).unwrap();
//     assert_eq!(actual, expected);
// }
