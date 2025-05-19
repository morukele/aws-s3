use std::fmt::Display;

use actix_multipart::MultipartError;
use actix_web::{http::StatusCode, Error as ActixError, HttpRequest, ResponseError};

#[derive(Debug)]
pub enum AWSError {
    UploadError(String),
    DownloadError(String),
}

impl Display for AWSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for AWSError {
    fn status_code(&self) -> StatusCode {
        match self {
            AWSError::UploadError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AWSError::DownloadError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub fn handle_multipart_error(err: MultipartError, _req: &HttpRequest) -> ActixError {
    AWSError::UploadError(format!("Multipart error: {}", err)).into()
}
