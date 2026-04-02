use std::path::Path;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Path as AxumPath, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use tokio_util::io::ReaderStream;
use utoipa::ToSchema;

use crate::config::Config;
use crate::error::AppError;
use crate::files::{self, FileInfo};

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RenameRequest {
    new_name: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/files",
    responses(
        (status = 200, description = "List of scanned files", body = Vec<FileInfo>)
    )
)]
pub async fn list_files(
    State(config): State<Arc<Config>>,
) -> Result<Json<Vec<FileInfo>>, AppError> {
    let entries = files::list_files(Path::new(&config.paths.output_dir)).await?;
    Ok(Json(entries))
}

#[utoipa::path(
    get,
    path = "/api/v1/files/{filename}",
    params(("filename" = String, Path, description = "File name")),
    responses((status = 200, description = "File data"))
)]
pub async fn download_file(
    State(config): State<Arc<Config>>,
    AxumPath(filename): AxumPath<String>,
) -> Result<impl IntoResponse, AppError> {
    let path = files::safe_path(Path::new(&config.paths.output_dir), &filename)?;
    if tokio::fs::metadata(&path).await.is_err() {
        return Err(AppError::NotFound(format!("File '{}' does not exist", filename)));
    }

    let mime = mime_guess::from_path(&path)
        .first_or_octet_stream()
        .to_string();

    let file = tokio::fs::File::open(&path).await?;
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok((
        [
            (header::CONTENT_TYPE, mime),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", filename),
            ),
        ],
        body,
    ))
}

#[utoipa::path(
    put,
    path = "/api/v1/files/{filename}",
    params(("filename" = String, Path, description = "File name")),
    request_body = RenameRequest,
    responses((status = 200, description = "Renamed"))
)]
pub async fn rename_file(
    State(config): State<Arc<Config>>,
    AxumPath(filename): AxumPath<String>,
    Json(body): Json<RenameRequest>,
) -> Result<Json<String>, AppError> {
    let old_path = files::safe_path(Path::new(&config.paths.output_dir), &filename)?;
    if tokio::fs::metadata(&old_path).await.is_err() {
        return Err(AppError::NotFound(format!("File '{}' does not exist", filename)));
    }
    let new_path = files::safe_path(Path::new(&config.paths.output_dir), &body.new_name)?;
    tokio::fs::rename(&old_path, &new_path).await?;

    // Also rename thumbnail if it exists
    let thumb_dir = Path::new(&config.paths.thumbnail_dir);
    let old_thumb = thumb_dir.join(&filename);
    if tokio::fs::metadata(&old_thumb).await.is_ok() {
        let new_thumb = thumb_dir.join(&body.new_name);
        let _ = tokio::fs::rename(&old_thumb, &new_thumb).await;
    }

    Ok(Json("200".to_string()))
}

#[utoipa::path(
    delete,
    path = "/api/v1/files/{filename}",
    params(("filename" = String, Path, description = "File name")),
    responses((status = 200, description = "Deleted file info", body = FileInfo))
)]
pub async fn delete_file(
    State(config): State<Arc<Config>>,
    AxumPath(filename): AxumPath<String>,
) -> Result<Json<FileInfo>, AppError> {
    let path = files::safe_path(Path::new(&config.paths.output_dir), &filename)?;
    let metadata = match tokio::fs::metadata(&path).await {
        Ok(m) => m,
        Err(_) => return Err(AppError::NotFound(format!("File '{}' does not exist", filename))),
    };
    let ext = Path::new(&filename)
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    let modified: chrono::DateTime<chrono::Utc> = metadata
        .modified()
        .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
        .into();

    let info = FileInfo {
        fullname: path.to_string_lossy().to_string(),
        name: filename.clone(),
        extension: ext,
        path: config.paths.output_dir.clone(),
        size: metadata.len(),
        size_string: files::format_size(metadata.len()),
        last_modified: modified.to_rfc3339(),
        is_directory: false,
    };

    tokio::fs::remove_file(&path).await?;

    // Also delete thumbnail if it exists
    let thumb_path = Path::new(&config.paths.thumbnail_dir).join(&filename);
    if tokio::fs::metadata(&thumb_path).await.is_ok() {
        let _ = tokio::fs::remove_file(&thumb_path).await;
    }

    Ok(Json(info))
}

#[utoipa::path(
    get,
    path = "/api/v1/files/{filename}/thumbnail",
    params(("filename" = String, Path, description = "File name")),
    responses((status = 200, description = "Thumbnail image"))
)]
pub async fn get_thumbnail(
    State(config): State<Arc<Config>>,
    AxumPath(filename): AxumPath<String>,
) -> Result<impl IntoResponse, AppError> {
    files::validate_filename(&filename)?;
    let thumb_path = Path::new(&config.paths.thumbnail_dir).join(&filename);
    if tokio::fs::metadata(&thumb_path).await.is_err() {
        return Err(AppError::NotFound("Thumbnail not found".to_string()));
    }

    let data = tokio::fs::read(&thumb_path).await?;
    Ok(([(header::CONTENT_TYPE, "image/jpeg".to_string())], data))
}
