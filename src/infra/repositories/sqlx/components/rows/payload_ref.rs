use anyhow::Context;
use sqlx::prelude::FromRow;

use crate::core::components::{
    models::payload::ComponentPayload,
    repositories::component_payload_resolver::ComponentPayloadRef,
};

#[derive(FromRow)]
pub struct ComponentPayloadRefRow {
    pub component_id: u32,
    pub version_id: u32,
    pub payload_str: String,
}

impl TryFrom<ComponentPayloadRefRow> for ComponentPayloadRef {
    type Error = anyhow::Error;

    fn try_from(row: ComponentPayloadRefRow) -> Result<Self, Self::Error> {
        let payload: ComponentPayload =
            serde_json::from_str(&row.payload_str).with_context(|| {
                format!(
                    "Failed to parse ComponentPayload JSON for component_id={}",
                    row.component_id
                )
            })?;

        Ok(Self {
            component_id: row.component_id,
            version_id: row.version_id,
            payload,
        })
    }
}
