use anyhow::Ok;

use crate::{configs::{self, server::ServerConfig}, utils, web::routes};

pub async fn run() -> anyhow::Result<()> {
    utils::loggers::setup_tracing();

    let config = configs::Config::from_env();
    let addr = config.get_addr();
    let app = routes::build().await?;

    tracing::info!("Server starting up on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
