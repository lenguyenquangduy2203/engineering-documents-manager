use async_trait::async_trait;

use crate::core::documents::repositories::DocumentLayoutsModifier;

use super::SqliteDocumentsRepository;

#[async_trait]
impl DocumentLayoutsModifier for SqliteDocumentsRepository {
    async fn replace_layouts(&self, doc_id: u32, version_ids: &[u32]) -> anyhow::Result<()> {
        let json_ids = serde_json::to_string(version_ids)?;
        let mut tx = self.dbc.begin().await?;

        sqlx::query!(
            r#"
            DELETE FROM document_layouts
            WHERE document_id = ?
                AND component_version_id IN (
                    SELECT value FROM json_each(?)
                )
            "#,
            doc_id,
            json_ids,
        )
        .execute(&mut *tx)
        .await?;

        if version_ids.is_empty() {
            tx.commit().await?;

            return Ok(());
        }

        let values: Vec<(u32, u32, usize)> = version_ids.iter()
            .enumerate()
            .map(|(index, id)| (doc_id, *id, index))
            .collect();
        let json_values = serde_json::to_string(&values)?;

        sqlx::query!(
            r#"
            INSERT INTO document_layouts (document_id, component_version_id, position)
            SELECT
                json_extract(value, '$[0]'),
                json_extract(value, '$[1]'),
                json_extract(value, '$[2]')
            FROM json_each(?)
            "#,
            json_values
        )
        .execute(&mut *tx)
        .await?;
            
        tx.commit().await?;

        Ok(())
    }
}
