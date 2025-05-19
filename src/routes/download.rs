use std::time::Duration;

use actix_web::{web, HttpResponse};
use aws_sdk_s3::{presigning::PresigningConfig, Client};

use crate::{entities::DownloadResponse, error::AWSError};

pub async fn download_handler(
    client: web::Data<Client>,
    key: web::Path<String>,
) -> Result<HttpResponse, AWSError> {
    let key = key.into_inner();
    let bucket_name = std::env::var("AWS_BUCKET_NAME").expect("unable to find s3 bucket name");
    let result = client.get_object().bucket(bucket_name).key(key);

    let presign_config = PresigningConfig::expires_in(Duration::from_secs(600))
        .map_err(|_| AWSError::DownloadError("Unable to generate presinged config".to_string()))?;

    let url = result
        .presigned(presign_config)
        .await
        .map_err(|_| AWSError::UploadError("Unable to generate presigned URL".to_string()))?;

    Ok(HttpResponse::Ok().json(DownloadResponse {
        url: url.uri().to_string(),
    }))
}
