use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};

use crate::core::components::{
    errors::wrapper::ComponentError,
    models::{payload::ComponentPayload, values::version::Version},
};

#[derive(Deserialize, Serialize, Clone, FromRow)]
pub struct Component<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    #[serde(default)]
    pub version: Version,
    pub title: String,
    pub payload: T,
}

impl Component<ComponentPayload> {
    pub fn apply_changes(
        self,
        incoming: Component<ComponentPayload>,
    ) -> Result<Self, ComponentError> {
        let current_type = self.payload.get_identifier();
        let incoming_type = incoming.payload.get_identifier();

        if current_type != incoming_type {
            return Err(ComponentError::IncompatibleType {
                expected: current_type,
                received: incoming_type,
            });
        }

        let updated_title = if incoming.title != self.title {
            incoming.title
        } else {
            self.title
        };

        std::result::Result::Ok(Component {
            id: self.id,
            version: self.version.next(),
            title: updated_title,
            payload: incoming.payload,
        })
    }
}

impl<T> From<Component<Json<T>>> for Component<T> {
    fn from(value: Component<Json<T>>) -> Self {
        Self {
            id: value.id,
            version: value.version,
            title: value.title,
            payload: value.payload.0,
        }
    }
}
