mod context;
mod components;

use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{infra::{dbc::sqlx::get_conn, repositories::sqlx::components::SqliteComponentRepository}, web::routes::context::Context};

pub async fn build() -> anyhow::Result<Router> {
    let dbc = Arc::new(get_conn().await?);
    let component_repository = Arc::new(SqliteComponentRepository::new(dbc.clone()));
    let ctx = Context::new(component_repository);
    let router = Router::new()
        .merge(components::build())
        .merge(Router::new().route("/", get(root_handler)))
        .with_state(ctx);

    Ok(router)
}

async fn root_handler() -> &'static str {
    "Hello, World!"
}