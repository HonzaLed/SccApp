use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SaltXML {
    pub status: String,
    pub salt: Option<String>,
    pub code: Option<String>,
    pub app_version: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LoginXML {
    pub status: String,
    pub token: Option<String>,
    pub code: Option<String>,
    pub app_version: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FileLinkXML {
    pub status: String,
    pub link: Option<String>,
    pub code: Option<String>,
    pub app_version: i32,
}