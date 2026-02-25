//! Local media file management for the approval queue.
//!
//! Stores uploaded media files on disk under `{data_dir}/media/` and
//! provides read/cleanup helpers. Media files are temporary — they exist
//! between upload and approval/rejection, then get cleaned up.

use std::path::{Path, PathBuf};

use crate::error::StorageError;
use crate::x_api::types::{ImageFormat, MediaType};

/// A locally stored media file.
#[derive(Debug, Clone)]
pub struct LocalMedia {
    /// Absolute path to the stored file.
    pub path: String,
    /// Detected media type.
    pub media_type: MediaType,
    /// File size in bytes.
    pub size: u64,
}

/// Store media data to disk under `{data_dir}/media/{uuid}.{ext}`.
///
/// Creates the media directory if it doesn't exist.
pub async fn store_media(
    data_dir: &Path,
    data: &[u8],
    _filename: &str,
    media_type: MediaType,
) -> Result<LocalMedia, StorageError> {
    let media_dir = data_dir.join("media");
    tokio::fs::create_dir_all(&media_dir)
        .await
        .map_err(|e| StorageError::Query {
            source: sqlx::Error::Io(e),
        })?;

    let ext = extension_for_type(media_type);
    let uuid = uuid_v4();
    let file_name = format!("{uuid}.{ext}");
    let file_path = media_dir.join(&file_name);

    tokio::fs::write(&file_path, data)
        .await
        .map_err(|e| StorageError::Query {
            source: sqlx::Error::Io(e),
        })?;

    Ok(LocalMedia {
        path: file_path.to_string_lossy().to_string(),
        media_type,
        size: data.len() as u64,
    })
}

/// Read media data from a local file path.
pub async fn read_media(path: &str) -> Result<Vec<u8>, StorageError> {
    tokio::fs::read(path)
        .await
        .map_err(|e| StorageError::Query {
            source: sqlx::Error::Io(e),
        })
}

/// Delete local media files. Errors are logged but not propagated.
pub async fn cleanup_media(paths: &[String]) {
    for path in paths {
        if let Err(e) = tokio::fs::remove_file(path).await {
            tracing::warn!(path = %path, error = %e, "Failed to clean up media file");
        }
    }
}

/// Detect media type from filename extension or content type string.
pub fn detect_media_type(filename: &str, content_type: Option<&str>) -> Option<MediaType> {
    // Try content type first.
    if let Some(ct) = content_type {
        match ct {
            "image/jpeg" => return Some(MediaType::Image(ImageFormat::Jpeg)),
            "image/png" => return Some(MediaType::Image(ImageFormat::Png)),
            "image/webp" => return Some(MediaType::Image(ImageFormat::Webp)),
            "image/gif" => return Some(MediaType::Gif),
            "video/mp4" => return Some(MediaType::Video),
            _ => {}
        }
    }

    // Fall back to extension.
    let lower = filename.to_lowercase();
    if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        Some(MediaType::Image(ImageFormat::Jpeg))
    } else if lower.ends_with(".png") {
        Some(MediaType::Image(ImageFormat::Png))
    } else if lower.ends_with(".webp") {
        Some(MediaType::Image(ImageFormat::Webp))
    } else if lower.ends_with(".gif") {
        Some(MediaType::Gif)
    } else if lower.ends_with(".mp4") {
        Some(MediaType::Video)
    } else {
        None
    }
}

/// Get file extension for a media type.
fn extension_for_type(media_type: MediaType) -> &'static str {
    match media_type {
        MediaType::Image(ImageFormat::Jpeg) => "jpg",
        MediaType::Image(ImageFormat::Png) => "png",
        MediaType::Image(ImageFormat::Webp) => "webp",
        MediaType::Gif => "gif",
        MediaType::Video => "mp4",
    }
}

/// Generate a simple UUID v4-like string using rand.
fn uuid_v4() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: [u8; 16] = rng.gen();
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
    )
}

/// Validate that a file path is under the expected media directory (path traversal protection).
pub fn is_safe_media_path(path: &str, data_dir: &Path) -> bool {
    let media_dir = data_dir.join("media");
    match PathBuf::from(path).canonicalize() {
        Ok(canonical) => canonical.starts_with(&media_dir),
        // If the file doesn't exist yet, check prefix.
        Err(_) => Path::new(path).starts_with(&media_dir),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_media_type_from_content_type() {
        assert_eq!(
            detect_media_type("photo.bin", Some("image/jpeg")),
            Some(MediaType::Image(ImageFormat::Jpeg))
        );
        assert_eq!(
            detect_media_type("x", Some("image/gif")),
            Some(MediaType::Gif)
        );
        assert_eq!(
            detect_media_type("x", Some("video/mp4")),
            Some(MediaType::Video)
        );
    }

    #[test]
    fn detect_media_type_from_extension() {
        assert_eq!(
            detect_media_type("photo.jpg", None),
            Some(MediaType::Image(ImageFormat::Jpeg))
        );
        assert_eq!(
            detect_media_type("photo.JPEG", None),
            Some(MediaType::Image(ImageFormat::Jpeg))
        );
        assert_eq!(
            detect_media_type("image.png", None),
            Some(MediaType::Image(ImageFormat::Png))
        );
        assert_eq!(
            detect_media_type("pic.webp", None),
            Some(MediaType::Image(ImageFormat::Webp))
        );
        assert_eq!(detect_media_type("ani.gif", None), Some(MediaType::Gif));
        assert_eq!(detect_media_type("clip.mp4", None), Some(MediaType::Video));
        assert_eq!(detect_media_type("file.txt", None), None);
    }

    #[tokio::test]
    async fn store_and_read_media() {
        let dir = tempfile::tempdir().expect("temp dir");
        let data = b"fake image data";

        let media = store_media(
            dir.path(),
            data,
            "test.jpg",
            MediaType::Image(ImageFormat::Jpeg),
        )
        .await
        .expect("store");

        assert!(media.path.ends_with(".jpg"));
        assert_eq!(media.size, data.len() as u64);

        let read_back = read_media(&media.path).await.expect("read");
        assert_eq!(read_back, data);
    }

    #[tokio::test]
    async fn cleanup_removes_files() {
        let dir = tempfile::tempdir().expect("temp dir");
        let data = b"temp media";

        let media = store_media(
            dir.path(),
            data,
            "temp.png",
            MediaType::Image(ImageFormat::Png),
        )
        .await
        .expect("store");

        assert!(Path::new(&media.path).exists());
        cleanup_media(&[media.path.clone()]).await;
        assert!(!Path::new(&media.path).exists());
    }
}
