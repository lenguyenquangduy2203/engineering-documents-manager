mod handlers;
mod payloads;

use axum::{Router, routing::{delete, get, patch, post, put}};

use crate::web::routes::{
    context::Context, 
    documents::handlers::{
        add_new_document, get_all_documents_with_layouts_order, get_document_with_layouts_by_id, 
        partially_update_document_by_id, publish_document_by_id, remove_document_by_id, update_document_layouts_by_id
    }
};

pub fn build() -> Router<Context> {
    Router::new()
        .route("/documents", post(add_new_document::handler))
        .route("/documents", get(get_all_documents_with_layouts_order::handler))
        .route("/documents/{id}", get(get_document_with_layouts_by_id::handler))
        .route("/documents/{id}", patch(partially_update_document_by_id::handler))
        .route("/documents/{id}", delete(remove_document_by_id::handler))
        .route("/documents/{id}/layouts", put(update_document_layouts_by_id::handler))
        .route("/documents/{id}/publish", post(publish_document_by_id::handler))
}
