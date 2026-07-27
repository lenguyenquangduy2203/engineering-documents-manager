use serde::{Deserialize, Serialize};

use crate::core::documents::models::{doc::Document, doc_types::DocTypes};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateDocRequest {
    pub doc_type: DocTypes,
    pub title: String,
}

impl From<CreateDocRequest> for Document {
    fn from(req: CreateDocRequest) -> Self {
        Self::new(req.doc_type, &req.title)
    }
}

#[derive(Serialize)]
pub struct CreatedResponse {
    pub id: u32,
}
