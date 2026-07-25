mod context;
mod components;
mod documents;

use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{infra::{dbc::sqlx::get_conn, repositories::sqlx::{components::SqliteComponentRepository, documents::SqliteDocumentsRepository}}, web::routes::context::Context};

pub async fn build() -> anyhow::Result<Router> {
    let dbc = Arc::new(get_conn().await?);

    let component_repository = Arc::new(SqliteComponentRepository::new(dbc.clone()));
    let document_repository = Arc::new(SqliteDocumentsRepository::new(dbc));
    
    let ctx = Context::new(
        component_repository.clone(),
        component_repository.clone(),
        component_repository,
        document_repository.clone(),
        document_repository.clone(),
        document_repository.clone(),
        document_repository
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