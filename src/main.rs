use actix_multipart::form::MultipartFormConfig;
use actix_web::{web, App, HttpServer};
use aws_sdk_s3::Client;
use error::handle_multipart_error;
use std::net::TcpListener;

mod entities;
mod error;
mod routes;
mod utils;

use routes::{download_handler, upload_handler};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let config = aws_config::load_from_env().await;
    let s3_client = Client::new(&config);

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(s3_client.clone()))
            .app_data(
                MultipartFormConfig::default()
                    .total_limit(10 * 1024 * 1024 * 1024) // 10 GB
                    .memory_limit(10 * 1025 * 1024) // 10 MB
                    .error_handler(handle_multipart_error),
            )
            .route("/upload", web::post().to(upload_handler))
            .route("/download/{key:.*}", web::get().to(download_handler))
    })
    .listen(listener)?
    .run()
    .await
}
