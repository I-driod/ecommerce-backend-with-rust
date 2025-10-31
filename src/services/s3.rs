use std::env;

use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client, primitives::ByteStream};
use uuid::Uuid;
use crate::utils::error::{AppError, Result};

pub struct S3Service {
    client: Client,
    bucket_name: String,
}

impl S3Service {
    pub async fn new() -> Result<Self> {
        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
        let client = Client::new(&config);
        let bucket_name = env::var("AWS_S3_BUCKET_NAME")
            .map_err(|_| AppError::S3Error("AWS_S3_BUCKET_NAME must be set".to_string()))?;

        Ok(S3Service {
            client,
            bucket_name,
        })
    }

    ///Upload a file to s3 and return public URL
    pub async fn upload_image(
                &self,
        file_data: Vec<u8>,
        content_type: &str,
        folder: &str
    ) -> Result<String> {

        // Generate unique filename 
        let file_extension = Self::get_extension_from_mime(content_type);
        let unique_filename = format!("{}/{}.{}", folder, Uuid::new_v4(), file_extension);


     //convert file data to ByteStream 
     let byte_stream = ByteStream::from(file_data.clone());

     //Upload to S3
     self.client
     .put_object()
     .bucket(&self.bucket_name)
     .key(&unique_filename)
     .body(byte_stream)
     .content_type(
        content_type)
        .send()
        .await
        .map_err(|e|{
            tracing::error!("S3 upload error: {:?}", e);
            AppError::InternalError
        })?;


        //Generate public URL 
        let region = env::var("AWS_REGION").unwrap_or_else(
            |_| "us-east-1".to_string());
        let public_url = format!(
            "https://{}.s3.{}.amazonaws.com/{}",
            self.bucket_name,
            region,
            unique_filename
            
        );

          tracing::info!("✅ Image uploaded to S3: {}", public_url);

        Ok(public_url)
    
    }


    ///Upload multiple images 
    pub async  fn upload_multiple_images(
        &self,
        files: Vec<(Vec<u8>, String)>,
        folder: &str
    ) -> Result<Vec<String>>{

        let mut urls = Vec::new();

        for (file_data, content_type) in files {
            let url = self.upload_image(
                file_data, &content_type, folder).await?;
                urls.push(url);

        }

        Ok(urls)

    }

        /// Delete an image from S3
    pub async fn delete_image(&self, image_url: &str) -> Result<()> {
        // Extract key from URL
        let key = image_url
            .split(&self.bucket_name)
            .nth(1)
            .and_then(|s| s.split('/').skip(1).collect::<Vec<_>>().join("/").into())
            .ok_or_else(|| AppError::ValidationError("Invalid S3 URL".to_string()))?;

        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("S3 delete error: {:?}", e);
                AppError::InternalError
            })?;

        tracing::info!("🗑️  Image deleted from S3: {}", image_url);

        Ok(())
    }


    /// Get file extension from MIME type
    fn get_extension_from_mime(content_type: &str) -> &str {
        match content_type {
            "image/jpeg" | "image/jpg" => "jpg",
            "image/png" => "png",
            "image/gif" => "gif",
            "image/webp" => "webp",
            _ => "jpg",  // default
        }
    }
}