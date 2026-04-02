use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

use crate::config::Config;
use crate::device::{Device, epson_v600};
use crate::error::AppError;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContextResponse {
    pub devices: Vec<Device>,
    pub version: String,
    pub paper_sizes: Vec<PaperSize>,
    pub actions: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaperSize {
    pub name: String,
    pub dimensions: Dimensions,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Dimensions {
    pub x: f64,
    pub y: f64,
}

pub fn paper_sizes() -> Vec<PaperSize> {
    vec![
        PaperSize { name: "A3 (@:paper-size.portrait)".into(), dimensions: Dimensions { x: 297.0, y: 420.0 } },
        PaperSize { name: "A4 (@:paper-size.portrait)".into(), dimensions: Dimensions { x: 210.0, y: 297.0 } },
        PaperSize { name: "A5 (@:paper-size.portrait)".into(), dimensions: Dimensions { x: 148.0, y: 210.0 } },
        PaperSize { name: "A5 (@:paper-size.landscape)".into(), dimensions: Dimensions { x: 210.0, y: 148.0 } },
        PaperSize { name: "A6 (@:paper-size.portrait)".into(), dimensions: Dimensions { x: 105.0, y: 148.0 } },
        PaperSize { name: "A6 (@:paper-size.landscape)".into(), dimensions: Dimensions { x: 148.0, y: 105.0 } },
        PaperSize { name: "B3 (@:paper-size.portrait)".into(), dimensions: Dimensions { x: 353.0, y: 500.0 } },
        PaperSize { name: "B4 (@:paper-size.portrait)".into(), dimensions: Dimensions { x: 250.0, y: 353.0 } },
        PaperSize { name: "B5 (@:paper-size.portrait)".into(), dimensions: Dimensions { x: 176.0, y: 250.0 } },
        PaperSize { name: "B5 (@:paper-size.landscape)".into(), dimensions: Dimensions { x: 250.0, y: 176.0 } },
        PaperSize { name: "B6 (@:paper-size.portrait)".into(), dimensions: Dimensions { x: 125.0, y: 176.0 } },
        PaperSize { name: "B6 (@:paper-size.landscape)".into(), dimensions: Dimensions { x: 176.0, y: 125.0 } },
        PaperSize { name: "DIN D3 (@:paper-size.portrait)".into(), dimensions: Dimensions { x: 272.0, y: 385.0 } },
        PaperSize { name: "DIN D4 (@:paper-size.portrait)".into(), dimensions: Dimensions { x: 192.0, y: 272.0 } },
        PaperSize { name: "DIN D5 (@:paper-size.portrait)".into(), dimensions: Dimensions { x: 136.0, y: 192.0 } },
        PaperSize { name: "DIN D5 (@:paper-size.landscape)".into(), dimensions: Dimensions { x: 192.0, y: 136.0 } },
        PaperSize { name: "DIN D6 (@:paper-size.portrait)".into(), dimensions: Dimensions { x: 96.0, y: 136.0 } },
        PaperSize { name: "DIN D6 (@:paper-size.landscape)".into(), dimensions: Dimensions { x: 136.0, y: 96.0 } },
        PaperSize { name: "@:paper-size.letter (@:paper-size.portrait)".into(), dimensions: Dimensions { x: 216.0, y: 279.0 } },
        PaperSize { name: "@:paper-size.legal (@:paper-size.portrait)".into(), dimensions: Dimensions { x: 216.0, y: 356.0 } },
        PaperSize { name: "@:paper-size.tabloid (@:paper-size.portrait)".into(), dimensions: Dimensions { x: 279.0, y: 432.0 } },
        PaperSize { name: "@:paper-size.ledger (@:paper-size.portrait)".into(), dimensions: Dimensions { x: 432.0, y: 279.0 } },
        PaperSize { name: "@:paper-size.junior-legal (@:paper-size.portrait)".into(), dimensions: Dimensions { x: 127.0, y: 203.0 } },
        PaperSize { name: "@:paper-size.half-letter (@:paper-size.portrait)".into(), dimensions: Dimensions { x: 140.0, y: 216.0 } },
    ]
}

#[utoipa::path(
    get,
    path = "/api/v1/context",
    responses(
        (status = 200, description = "Device context", body = ContextResponse)
    )
)]
pub async fn get_context(
    State(_config): State<Arc<Config>>,
) -> Result<Json<ContextResponse>, AppError> {
    let device = epson_v600();
    Ok(Json(ContextResponse {
        devices: vec![device],
        version: env!("CARGO_PKG_VERSION").to_string(),
        paper_sizes: paper_sizes(),
        actions: vec![],
    }))
}
