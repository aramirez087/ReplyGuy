//! DB-free media upload function.
//!
//! Extracted from `tools/x_actions/media.rs` for use in the API profile.

use std::time::Instant;

use serde::Serialize;

use crate::contract::envelope::{ToolMeta, ToolResponse};
use crate::contract::error_code::ErrorCode;
use tuitbot_core::x_api::types::{ImageFormat, MediaType};
use tuitbot_core::x_api::XApiClient;

/// Upload a media file for attachment to tweets.
pub async fn upload_media(client: &dyn XApiClient, file_path: &str) -> String {
    let start = Instant::now();

    let media_type = match infer_media_type(file_path) {
        Some(mt) => mt,
        None => {
            let elapsed = start.elapsed().as_millis() as u64;
            return ToolResponse::error(
                ErrorCode::UnsupportedMediaType,
                format!(
                    "Unsupported file extension for: {file_path}. \
                     Supported: jpg, jpeg, png, webp, gif, mp4"
                ),
            )
            .with_meta(ToolMeta::new(elapsed))
            .to_json();
        }
    };

    let data = match tokio::fs::read(file_path).await {
        Ok(d) => d,
        Err(e) => {
            let elapsed = start.elapsed().as_millis() as u64;
            return ToolResponse::error(
                ErrorCode::FileReadError,
                format!("Failed to read file {file_path}: {e}"),
            )
            .with_meta(ToolMeta::new(elapsed))
            .to_json();
        }
    };

    let file_size = data.len();

    match client.upload_media(&data, media_type).await {
        Ok(media_id) => {
            let elapsed = start.elapsed().as_millis() as u64;
            #[derive(Serialize)]
            struct UploadResult {
                media_id: String,
                media_type: String,
                file_size_bytes: usize,
            }
            ToolResponse::success(UploadResult {
                media_id: media_id.0,
                media_type: media_type.mime_type().to_string(),
                file_size_bytes: file_size,
            })
            .with_meta(ToolMeta::new(elapsed))
            .to_json()
        }
        Err(e) => {
            let elapsed = start.elapsed().as_millis() as u64;
            ToolResponse::error(
                ErrorCode::MediaUploadError,
                format!("Media upload failed: {e}"),
            )
            .with_meta(ToolMeta::new(elapsed))
            .to_json()
        }
    }
}

/// Infer `MediaType` from a file path extension.
fn infer_media_type(path: &str) -> Option<MediaType> {
    let ext = path.rsplit('.').next()?.to_ascii_lowercase();
    match ext.as_str() {
        "jpg" | "jpeg" => Some(MediaType::Image(ImageFormat::Jpeg)),
        "png" => Some(MediaType::Image(ImageFormat::Png)),
        "webp" => Some(MediaType::Image(ImageFormat::Webp)),
        "gif" => Some(MediaType::Gif),
        "mp4" => Some(MediaType::Video),
        _ => None,
    }
}
