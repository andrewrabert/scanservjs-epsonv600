use std::sync::Arc;

use clap::Parser;

mod config;
mod device;
mod error;
mod files;
mod routes;
mod scanner;

#[derive(Parser)]
#[command(name = "scanservjs-epsonv600", version)]
struct Cli {
    /// Path to config file
    #[arg(short, long, default_value = "/etc/scanservjs/config.toml")]
    config: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config = config::Config::load(&cli.config).expect("Failed to load config");

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.log.level.parse().expect("Invalid log level")),
        )
        .init();

    let bind_addr = format!("{}:{}", config.server.host, config.server.port);
    let state = Arc::new(config);
    let app = routes::build_router(state);

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("Failed to bind");
    tracing::info!("Listening on {}", bind_addr);

    axum::serve(listener, app).await.expect("Server error");
}
