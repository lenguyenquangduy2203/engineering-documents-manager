use anyhow::Ok;
use sqlx::prelude::FromRow;

use crate::core::components::models::{
    payload::ComponentPayload, values::version::Version, wrapper::Component,
};

#[derive(FromRow)]
pub struct ComponentRow {
    pub id: u32,
    pub version: u32,
    pub title: String,
    pub payload: String,
}

impl TryFrom<ComponentRow> for Component<ComponentPayload> {
    type Error = anyhow::Error;

    fn try_from(row: ComponentRow) -> Result<Self, Self::Error> {
        Ok(Component {
            id: Some(row.id),
            version: Version(row.version),
            title: row.title,
            payload: serde_json::from_str(&row.payload)?,
        })
    }
}
