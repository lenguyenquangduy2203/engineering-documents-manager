use serde::{Deserialize, Serialize};

use crate::core::components::models::{values::version::Version, wrapper::Component};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateComponentRequest<T> {
    pub title: String,
    pub payload: T,
}

impl<T> From<CreateComponentRequest<T>> for Component<T> {
    fn from(req: CreateComponentRequest<T>) -> Self {
        Component {
            id: None,
            version: Version::default(),
            title: req.title,
            payload: req.payload,
        }
    }
}

#[derive(Serialize)]
pub struct CreatedResponse {
    pub id: u32,
}
