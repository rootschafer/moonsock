use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilelistChangedParam {
    pub action: FilelistAction,
    pub item: FilelistItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_item: Option<FilelistItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FilelistAction {
    #[serde(rename = "create_file")]
    CreateFile,
    #[serde(rename = "create_dir")]
    CreateDir,
    #[serde(rename = "delete_file")]
    DeleteFile,
    #[serde(rename = "delete_dir")]
    DeleteDir,
    #[serde(rename = "move_file")]
    MoveFile,
    #[serde(rename = "move_dir")]
    MoveDir,
    #[serde(rename = "modify_file")]
    ModifyFile,
    #[serde(rename = "root_update")]
    RootUpdate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilelistItem {
    pub path: String,
    pub root: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MoonResponse, JsonRpcVersion, 
        NotificationMethod, NotificationParam, 
        // FilelistChangedParam, FilelistAction, FilelistItem,
    };
    #[test]
    fn test_parse_notify_filelist_changed() {
        let json = r#"{
            "jsonrpc": "2.0",
            "method": "notify_filelist_changed",
            "params": [
                {
                    "action": "create_file",
                    "item": {
                        "path": "some/path",
                        "root": "root_name",
                        "size": 46458,
                        "modified": 545465,
                        "permissions": "rw"
                    },
                    "source_item": {
                        "path": "some/source/path",
                        "root": "root_name"
                    }
                }
            ]
        }"#;

        let expected = MoonResponse::Notification {
            jsonrpc: JsonRpcVersion::V2,
            method: NotificationMethod::NotifyFilelistChanged,
            params: Some(
                NotificationParam::FilelistChanged(vec![FilelistChangedParam {
                    action: FilelistAction::CreateFile,
                    item: FilelistItem {
                        path: "some/path".to_string(),
                        root: "root_name".to_string(),
                        size: Some(46458),
                        modified: Some(545465),
                        permissions: Some("rw".to_string()),
                    },
                    source_item: Some(FilelistItem {
                        path: "some/source/path".to_string(),
                        root: "root_name".to_string(),
                        size: None,
                        modified: None,
                        permissions: None,
                    }),
                }]),
            ),
        };

        let actual: MoonResponse = serde_json::from_str(json).unwrap();
        assert_eq!(actual, expected);
    }
}