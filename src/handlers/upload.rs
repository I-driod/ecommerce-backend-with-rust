use axum::{
    extract::{Multipart,},
    http::StatusCode,
    Json,
};
use serde::Serialize;


use crate::{
    middleware::auth::AuthUser,
    services::s3::S3Service,
    utils::error::{AppError, Result},
};

#[derive(Serialize)]
pub struct ImageUploadResponse {
    pub status: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct MultipleImageUploadResponse {
    pub status: String,
    pub urls: Vec<String>,
}

/// Upload a single product image
/// POST /api/upload/image
pub async fn upload_single_image(
    _auth: AuthUser,  // Require authentication
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<ImageUploadResponse>)> {
    let s3_service = S3Service::new().await?;

    // Extract file from multipart form
    if let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!("Multipart error: {:?}", e);
        AppError::ValidationError("Failed to read multipart data".to_string())
    })? {
        let content_type = field
            .content_type()
            .unwrap_or("image/jpeg")
            .to_string();

        // Validate content type
        if !content_type.starts_with("image/") {
            return Err(AppError::ValidationError(
                "Only image files are allowed".to_string(),
            ));
        }

        // Read file data
        let data = field.bytes().await.map_err(|e| {
            tracing::error!("Failed to read file bytes: {:?}", e);
            AppError::ValidationError("Failed to read file data".to_string())
        })?;

        // Check file size (max 5MB)
        if data.len() > 5 * 1024 * 1024 {
            return Err(AppError::ValidationError(
                "File size must be less than 5MB".to_string(),
            ));
        }

        // Upload to S3
        let url = s3_service
            .upload_image(data.to_vec(), &content_type, "products")
            .await?;

        Ok((
            StatusCode::CREATED,
            Json(ImageUploadResponse {
                status: "success".to_string(),
                url,
            }),
        ))
    } else {
        Err(AppError::ValidationError("No file provided".to_string()))
    }
}

/// Upload multiple product images
/// POST /api/upload/images
pub async fn upload_multiple_images(
    _auth: AuthUser,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<MultipleImageUploadResponse>)> {
    let s3_service = S3Service::new().await?;
    let mut files = Vec::new();

    // Extract all files from multipart form
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!("Multipart error: {:?}", e);
        AppError::ValidationError("Failed to read multipart data".to_string())
    })? {
        let content_type = field
            .content_type()
            .unwrap_or("image/jpeg")
            .to_string();

        // Validate content type
        if !content_type.starts_with("image/") {
            continue; // Skip non-image files
        }

        // Read file data
        let data = field.bytes().await.map_err(|e| {
            tracing::error!("Failed to read file bytes: {:?}", e);
            AppError::ValidationError("Failed to read file data".to_string())
        })?;

        // Check file size
        if data.len() > 5 * 1024 * 1024 {
            return Err(AppError::ValidationError(
                "Each file must be less than 5MB".to_string(),
            ));
        }

        files.push((data.to_vec(), content_type));
    }

    if files.is_empty() {
        return Err(AppError::ValidationError("No files provided".to_string()));
    }

    // Limit to 5 images
    if files.len() > 5 {
        return Err(AppError::ValidationError(
            "Maximum 5 images allowed".to_string(),
        ));
    }

    // Upload all files to S3
    let urls = s3_service
        .upload_multiple_images(files, "products")
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(MultipleImageUploadResponse {
            status: "success".to_string(),
            urls,
        }),
    ))
}
