use serde::{Deserialize, Serialize};

use crate::core::components::models::values::version::Version;

#[derive(Deserialize, Serialize, Clone)]
pub struct Component<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    #[serde(default)]
    pub version: Version,
    pub title: String,
    pub payload: T,
}
