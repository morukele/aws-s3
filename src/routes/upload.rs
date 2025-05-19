use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use futures_util::TryStreamExt;

use crate::entities::File;
use crate::error::AWSError;
use crate::utils::field_to_bytes;

pub async fn upload_handler(
    client: web::Data<Client>,
    mut payload: Multipart,
) -> Result<HttpResponse, AWSError> {
    let mut files = vec![];
    let bucket_name = std::env::var("AWS_BUCKET_NAME").expect("unable to find s3 bucket name");
    // iterate over multipart stream
    while let Some(field) = payload
        .try_next()
        .await
        .map_err(|_| AWSError::UploadError("Unable to extract multipart".to_string()))?
    {
        if let Some("files") = field.name() {
            let Some(content_disposition) = field.content_disposition() else {
                continue;
            };

            let file_name = content_disposition
                .get_filename()
                .unwrap_or_default()
                .to_owned();
            let content_type = field.content_type().unwrap().to_string();
            let key = uuid::Uuid::new_v4().to_string();

            let bytes = field_to_bytes(field).await.map_err(|_| {
                AWSError::UploadError("Unable to convert field to bytes".to_string())
            })?;

            // put files into file vector
            files.push(File {
                file_name,
                content_type,
                key,
                bytes,
                successful: false,
            });
        }
    }

    // upload all files in the vector into s3 bucket
    for file in &mut files {
        let body = ByteStream::from(file.bytes.to_vec());
        let _res = client
            .put_object()
            .bucket(&bucket_name)
            .body(body)
            .content_type(&file.content_type)
            .content_length(file.bytes.len() as i64)
            .key(&file.key)
            .send()
            .await;

        file.successful = true;
    }

    Ok(HttpResponse::Ok().json(files))
}
