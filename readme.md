# AWS S3 File Manager API

A Rust web API that provides a secure interface for working with AWS S3 buckets. This service allows you to upload and download files while keeping your S3 bucket private.

## Features

- File upload via multipart form
- Secure file downloads using presigned URLs
- Support for large files (up to 10GB)
- Environment-based configuration

## Prerequisites

- Rust toolchain
- AWS account with S3 access
- Valid AWS credentials (access key and secret)

## Setup

1. Clone the repository
2. Copy the environment template:

   ```bash
   cp .env.example .env
   ```

3. Configure your `.env` file with:

   ```env
   AWS_ACCESS_KEY_ID=your_access_key
   AWS_SECRET_ACCESS_KEY=your_secret_key
   AWS_REGION=your_region
   AWS_BUCKET_NAME=your_bucket_name
   ```

4. Set you AWS IAM policy with:

   ```json
   {
     "Effect": "Allow",
     "Action": ["s3:PutObject", "s3:GetObject"],
     "Resource": "arn:aws:s3:::my-demo-bucket/*"
   }
   ```

5. Build and run the project:

   ```bash
   cargo build
   cargo run
   ```

The server will start on `http://127.0.0.1:7878`

## API Endpoints

### Upload File

- **URL**: `/upload`
- **Method**: `POST`
- **Content-Type**: `multipart/form-data`
- **Body**: Form data with file
- **Response**:

  ```json
  [
    {
      "key": "file_key",
      "successful": true,
      "file_name": "file_name",
      "content_type": "application/file-type"
    }
  ]
  ```

### Download File

- **URL**: `/download/{key}`
- **Method**: `GET`
- **Response**:

  ```json
  {
    "url": "presigned-s3-url"
  }
  ```

## Configuration

The server is configured with:

- Maximum file size: 10GB
- Memory buffer: 10MB
- Presigned URL expiration: 10 minutes

## Development

Built with:

- actix-web - Web framework
- aws-sdk-s3 - AWS S3 SDK
- tokio - Async runtime

## Error Handling

The API returns appropriate HTTP status codes:

- 200: Success
- 400: Bad Request
- 404: File Not Found
- 500: Server Error
