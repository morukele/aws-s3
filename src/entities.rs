use bytes::Bytes;

use serde::Serialize;

#[derive(Serialize)]
pub struct File {
    pub key: String,
    pub successful: bool,
    pub file_name: String,
    pub content_type: String,
    #[serde(skip_serializing)]
    pub bytes: Bytes,
}

#[derive(Serialize)]
pub struct DownloadResponse {
    pub url: String,
}
