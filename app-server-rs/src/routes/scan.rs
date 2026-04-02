use std::path::Path;
use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::config::Config;
use crate::device::epson_v600;
use crate::error::AppError;
use crate::files::{self, FileInfo};
use crate::scanner::command::build_scan_command;
use crate::scanner::request::validate_params;
use crate::scanner::ScanRequest;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScanResponse {
    pub file_info: FileInfo,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PreviewResponse {
    pub content: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct WrapperOutput {
    scan_path: Option<String>,
    preview_path: Option<String>,
    thumbnail_path: Option<String>,
}

async fn run_scanimage(
    config: &Config,
    request: &ScanRequest,
    timeout_secs: u64,
) -> Result<WrapperOutput, AppError> {
    let device = epson_v600();
    validate_params(&request.params, &device)?;

    let args = build_scan_command(&config.scanner.command, &request.params);

    tracing::info!("Running: {:?}", args);

    let output = tokio::time::timeout(
        std::time::Duration::from_secs(timeout_secs),
        tokio::process::Command::new(&args[0])
            .args(&args[1..])
            .output(),
    )
    .await
    .map_err(|_| AppError::Internal("Scan timed out".to_string()))?
    .map_err(|e| AppError::Internal(format!("Failed to execute scanimage: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Internal(format!(
            "scanimage exited with code {}: {}",
            output.status.code().unwrap_or(-1),
            stderr
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let info: WrapperOutput = serde_json::from_str(&stdout).map_err(|e| {
        AppError::Internal(format!("Failed to parse scanimage output: {}: {}", e, stdout))
    })?;

    Ok(info)
}

#[utoipa::path(
    post,
    path = "/api/v1/scan",
    request_body = ScanRequest,
    responses(
        (status = 200, description = "Scan result", body = ScanResponse)
    )
)]
pub async fn scan(
    State(config): State<Arc<Config>>,
    Json(request): Json<ScanRequest>,
) -> Result<Json<ScanResponse>, AppError> {
    let info = run_scanimage(&config, &request, config.server.timeout_secs).await?;

    let scan_path = info
        .scan_path
        .ok_or_else(|| AppError::Internal("No scanPath in output".to_string()))?;
    let path = Path::new(&scan_path);
    let metadata = tokio::fs::metadata(path).await?;
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let ext = path
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    let dir = path
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    let modified: chrono::DateTime<chrono::Utc> = metadata
        .modified()
        .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
        .into();

    Ok(Json(ScanResponse {
        file_info: FileInfo {
            fullname: scan_path,
            name,
            extension: ext,
            path: dir,
            size: metadata.len(),
            size_string: files::format_size(metadata.len()),
            last_modified: modified.to_rfc3339(),
            is_directory: false,
        },
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/preview",
    request_body = ScanRequest,
    responses(
        (status = 200, description = "Preview created", body = PreviewResponse)
    )
)]
pub async fn create_preview(
    State(config): State<Arc<Config>>,
    Json(mut request): Json<ScanRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    request.params.is_preview = Some(true);
    request.params.resolution = config.scanner.preview_resolution;
    run_scanimage(&config, &request, config.server.timeout_secs).await?;
    Ok(Json(serde_json::json!({})))
}

#[utoipa::path(
    get,
    path = "/api/v1/preview",
    responses(
        (status = 200, description = "Current preview", body = PreviewResponse)
    )
)]
pub async fn get_preview(
    State(config): State<Arc<Config>>,
) -> Result<Json<PreviewResponse>, AppError> {
    read_preview(&config).await
}

async fn read_preview(config: &Config) -> Result<Json<PreviewResponse>, AppError> {
    let preview_path = Path::new(&config.paths.preview_dir).join("preview.jpg");
    let data = match tokio::fs::read(&preview_path).await {
        Ok(d) => d,
        Err(_) => return Ok(Json(PreviewResponse { content: String::new() })),
    };
    use base64::Engine;
    let encoded = base64::engine::general_purpose::STANDARD.encode(&data);
    Ok(Json(PreviewResponse { content: encoded }))
}

#[utoipa::path(
    delete,
    path = "/api/v1/preview",
    responses(
        (status = 200, description = "Preview deleted"))
)]
pub async fn delete_preview(
    State(config): State<Arc<Config>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let preview_path = Path::new(&config.paths.preview_dir).join("preview.jpg");
    let _ = tokio::fs::remove_file(&preview_path).await;
    Ok(Json(serde_json::json!({})))
}
