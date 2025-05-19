use actix_multipart::Field;
use bytes::{Bytes, BytesMut};
use futures_util::StreamExt;

pub async fn field_to_bytes(mut field: Field) -> Result<Bytes, actix_web::Error> {
    let mut bytes = BytesMut::new();

    while let Some(chunk) = field.next().await {
        let data = chunk?;
        bytes.extend_from_slice(&data);
    }

    Ok(bytes.freeze()) // converts BytesMut to Bytes
}
