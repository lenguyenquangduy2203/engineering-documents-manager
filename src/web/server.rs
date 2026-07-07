use axum::{Router, routing::get};

use crate::config;

pub async fn run() {
    config::setup_logging();

    let config = config::Config::from_env();
    let addr =format!("{}:{}", config.host, config.port);
    let app = build_router();

    tracing::info!("Server starting up on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn build_router() -> Router {
    Router::new().route("/", get(root_handler))
}

async fn root_handler() -> &'static str {
    "Hello, World!"
}
