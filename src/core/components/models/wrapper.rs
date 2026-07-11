use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Component<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub title: String,
    pub payload: T,
}
