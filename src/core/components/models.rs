use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Component<T> {
    pub title: String,
    pub component_type: String,
    pub payload: T,
}
