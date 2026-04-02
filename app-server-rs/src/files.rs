use std::path::{Path, PathBuf};

use serde::Serialize;
use utoipa::ToSchema;

use crate::error::AppError;

const FORBIDDEN_CHARS: &[char] = &['/', '\\', '?', '%', '*', ':', '|', '"', '<', '>', ';', '='];

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    pub fullname: String,
    pub name: String,
    pub extension: String,
    pub path: String,
    pub size: u64,
    pub size_string: String,
    pub last_modified: String,
    pub is_directory: bool,
}

pub fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let mut size = bytes as f64;
    for unit in UNITS {
        if size < 1024.0 {
            return format!("{} {}", (size).round() as u64, unit);
        }
        size /= 1024.0;
    }
    format!("{} TB", (size).round() as u64)
}

pub fn validate_filename(name: &str) -> Result<(), AppError> {
    if name.is_empty() {
        return Err(AppError::BadRequest("Filename cannot be empty".to_string()));
    }
    if name.contains("..") {
        return Err(AppError::BadRequest("Parent paths disallowed".to_string()));
    }
    if name.starts_with('/') {
        return Err(AppError::BadRequest("Absolute paths disallowed".to_string()));
    }
    if name.contains('\0') {
        return Err(AppError::BadRequest("Null bytes disallowed".to_string()));
    }
    if name.contains(FORBIDDEN_CHARS) {
        return Err(AppError::BadRequest(
            "Name cannot contain illegal characters".to_string(),
        ));
    }
    Ok(())
}

pub fn safe_path(dir: &Path, filename: &str) -> Result<PathBuf, AppError> {
    validate_filename(filename)?;
    Ok(dir.join(filename))
}

pub async fn list_files(dir: &Path) -> Result<Vec<FileInfo>, AppError> {
    let mut entries = Vec::new();
    let mut read_dir = tokio::fs::read_dir(dir).await?;
    while let Some(entry) = read_dir.next_entry().await? {
        let metadata = entry.metadata().await?;
        if metadata.is_file() {
            let name = entry.file_name().to_string_lossy().to_string();
            let ext = Path::new(&name)
                .extension()
                .map(|e| format!(".{}", e.to_string_lossy()))
                .unwrap_or_default();
            let modified = metadata
                .modified()
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
            let datetime: chrono::DateTime<chrono::Utc> = modified.into();

            entries.push(FileInfo {
                fullname: entry.path().to_string_lossy().to_string(),
                name: name.clone(),
                extension: ext,
                path: dir.to_string_lossy().to_string(),
                size: metadata.len(),
                size_string: format_size(metadata.len()),
                last_modified: datetime.to_rfc3339(),
                is_directory: false,
            });
        }
    }
    entries.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_size_bytes() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1 KB");
        assert_eq!(format_size(1536), "2 KB");
        assert_eq!(format_size(1_048_576), "1 MB");
        assert_eq!(format_size(1_073_741_824), "1 GB");
    }

    #[test]
    fn validate_filename_rejects_traversal() {
        assert!(validate_filename("../etc/passwd").is_err());
        assert!(validate_filename("foo/../../bar").is_err());
        assert!(validate_filename("/absolute/path").is_err());
        assert!(validate_filename("file\0name").is_err());
    }

    #[test]
    fn validate_filename_rejects_forbidden_chars() {
        for c in ['/', '\\', '?', '%', '*', ':', '|', '"', '<', '>', ';', '='] {
            let name = format!("scan{c}.jpg");
            assert!(validate_filename(&name).is_err(), "Should reject '{c}'");
        }
    }

    #[test]
    fn validate_filename_accepts_valid_names() {
        assert!(validate_filename("scan_20220115_211429.jpg").is_ok());
        assert!(validate_filename("my scan (1).tif").is_ok());
        assert!(validate_filename("photo.jpeg-xl").is_ok());
    }

    #[test]
    fn safe_path_stays_within_dir() {
        let dir = Path::new("/data/scans");
        assert!(safe_path(dir, "scan.jpg").is_ok());
        assert!(safe_path(dir, "../etc/passwd").is_err());
    }
}
