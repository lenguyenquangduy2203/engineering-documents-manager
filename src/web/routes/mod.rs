mod context;
mod components;
mod documents;

use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{configs::{self, server::ServerConfig}, infra::{dbc::sqlx::get_conn, rendering::services::MarkdownRenderService, repositories::sqlx::{components::SqliteComponentRepository, documents::SqliteDocumentsRepository}}, web::routes::context::Context};

pub async fn build() -> anyhow::Result<Router> {
    let config = configs::Config::from_env();
    let dbc = Arc::new(get_conn().await?);

    let component_repository = Arc::new(SqliteComponentRepository::new(dbc.clone()));
    let document_repository = Arc::new(SqliteDocumentsRepository::new(dbc));
    let export_service = Arc::new(MarkdownRenderService::new(config.get_export_dir()));
    
    let ctx = Context::new(
        component_repository.clone(),
        component_repository.clone(),
        component_repository,
        document_repository.clone(),
        document_repository.clone(),
        document_repository.clone(),
        document_repository,
        export_service
    );

    let router = Router::new()
        .merge(components::build())
        .merge(documents::build())
        .merge(Router::new().route("/", get(root_handler)))
        .with_state(ctx);

    Ok(router)
}

async fn root_handler() -> &'static str {
    "Hello, World!"
}