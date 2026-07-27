mod payloads;
mod handlers;

use axum::{
    Router, 
    routing::{
        delete, get, 
        post, put
    }
};

use crate::web::routes::{
    components::handlers::{
        add_new_component, get_all_latest_components, get_latest_component, 
        remove_component_by_id, update_component_by_id
    }, 
    context::Context
};

pub fn build() -> Router<Context> {
    Router::new()
        .route("/components", post(add_new_component::handler))
        .route("/components", get(get_all_latest_components::handler))
        .route("/components/{id}", get(get_latest_component::handler))
        .route("/components/{id}", put(update_component_by_id::handler))
        .route("/components/{id}", delete(remove_component_by_id::handler))
}
