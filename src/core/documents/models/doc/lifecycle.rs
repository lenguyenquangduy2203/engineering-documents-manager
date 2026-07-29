use anyhow::anyhow;
use chrono::{DateTime, Utc};

use crate::core::documents::errors::doc_publication::DocumentPublishingError;

use super::Document;

impl Document {
    pub fn marked_for_publishing(&mut self) -> Result<(), DocumentPublishingError> {
        if self.layout_version_ids.is_empty() {
            return Err(DocumentPublishingError::EmptyLayout);
        }

        self.status = self.status.transition_to_publishing()?;

        Ok(())
    }

    pub fn finalize_publication(&mut self) -> anyhow::Result<DateTime<Utc>> {
        if self.layout_version_ids.is_empty() {
            return Err(anyhow!("Cannot publish an empty document layout."));
        }

        self.status = self.status.transition_to_published()?;

        Ok(Utc::now())
    }

    pub fn mark_failed(&mut self) -> anyhow::Result<()> {
        self.status = self.status.transition_to_failed()?;

        Ok(())
    }
}
