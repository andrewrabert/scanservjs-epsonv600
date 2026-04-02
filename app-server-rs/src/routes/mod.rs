use std::sync::Arc;

use axum::{Router, routing::{get, post}};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::Config;
use crate::device::{Device, Feature};
use crate::files::FileInfo;
use crate::scanner::{ScanRequest, ScanParams};

pub mod context;
pub mod files;
pub mod scan;

#[derive(OpenApi)]
#[openapi(
    paths(
        context::get_context,
        files::list_files,
        files::download_file,
        files::rename_file,
        files::delete_file,
        files::get_thumbnail,
        scan::scan,
        scan::create_preview,
        scan::get_preview,
        scan::delete_preview,
    ),
    components(schemas(
        FileInfo,
        Device,
        Feature,
        ScanRequest,
        ScanParams,
        context::ContextResponse,
        context::PaperSize,
        context::Dimensions,
        scan::ScanResponse,
        scan::PreviewResponse,
        files::RenameRequest,
    ))
)]
struct ApiDoc;

pub fn build_router(state: Arc<Config>) -> Router {
    let client_dir = state.paths.client_dir.clone();

    Router::new()
        .route("/api/v1/context", get(context::get_context))
        .route("/api/v1/files", get(files::list_files))
        .route(
            "/api/v1/files/{filename}",
            get(files::download_file)
                .put(files::rename_file)
                .delete(files::delete_file),
        )
        .route(
            "/api/v1/files/{filename}/thumbnail",
            get(files::get_thumbnail),
        )
        .route("/api/v1/scan", post(scan::scan))
        .route(
            "/api/v1/preview",
            get(scan::get_preview)
                .post(scan::create_preview)
                .delete(scan::delete_preview),
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .fallback_service(
            ServeDir::new(&client_dir)
                .append_index_html_on_directories(true)
                .not_found_service(ServeFile::new(format!("{}/index.html", client_dir))),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
