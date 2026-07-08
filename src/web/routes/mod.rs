mod context;

use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{infra::dbc::sqlx::get_conn, web::routes::context::Context};

pub async fn build() -> anyhow::Result<Router> {
    let dbc = Arc::new(get_conn().await?);
    let global_ctx = Context::new(dbc);
    let router = Router::new().route("/", get(root_handler)).with_state(global_ctx);

    Ok(router)
}

async fn root_handler() -> &'static str {
    "Hello, World!"
}