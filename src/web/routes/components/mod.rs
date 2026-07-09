use axum::{Router, routing::post};
use crate::web::routes::{context::Context};

mod design_specs;

pub fn build() -> Router<Context> {
    Router::new()
        .route("/specs", post(design_specs::add_new_design_spec))
}