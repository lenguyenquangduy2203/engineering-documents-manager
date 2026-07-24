use std::sync::Arc;

use anyhow::anyhow;

use crate::{core::{
    components::repositories::ComponentPayloadResolver, documents::{models::doc::Document, repositories::{DocumentPublishParams, DocumentPublisher, DocumentsResolver}},
}, infra::rendering::services::MarkdownRenderService};

pub struct DocumentPublishingService;

type DocumentPublishingServiceDepsTuple = (
    Arc<dyn DocumentsResolver>,
    Arc<dyn ComponentPayloadResolver>,
    Arc<dyn DocumentPublisher>,
);

impl DocumentPublishingService {
    pub async fn publish_document(
        (documents_resolver, component_payload_resolver, document_publisher): DocumentPublishingServiceDepsTuple,
        doc_id: u32,
    ) -> anyhow::Result<()> {
        let mut document = documents_resolver.find_doc_by_id(doc_id).await?
            .ok_or_else(|| anyhow!("Document not found for marking publishing"))?;

        document.marked_for_publishing()?;

        document_publisher.update_doc_publication(DocumentPublishParams { 
            id: doc_id, 
            status: document.status.to_string(), 
            published_at: None, 
        }).await?;

        let component_payload_resolver = Arc::clone(&component_payload_resolver);
        let document_publisher = Arc::clone(&document_publisher);

        tokio::spawn(async move {
            match Self::execute_publish_task(
                component_payload_resolver.as_ref(),
                document_publisher.as_ref(), 
                doc_id,
                &mut document
            ).await {
                Ok(_) => {
                    tracing::info!(doc_id = doc_id, "Successfully published document & saved file");
                }
                Err(err) => {
                    tracing::error!(target: "publishing", error = %err, doc_id = doc_id, "Background publish failed");
                    document.forced_failed();
                    // Cleanup / revert status back to Draft/Failed so the user can try again
                    if let Err(revert_err) = document_publisher.update_doc_publication(DocumentPublishParams { 
                        id: doc_id, 
                        status: document.status.to_string(), 
                        published_at: None
                    }).await {
                        tracing::error!(doc_id = doc_id, error = %revert_err, "Failed to mark document as Failed");
                    }
                }
            }
        });

        Ok(())
    }

    async fn execute_publish_task(
        component_payload_resolver: &dyn ComponentPayloadResolver,
        document_publisher: &dyn DocumentPublisher,
        doc_id: u32,
        document: &mut Document,
    ) -> anyhow::Result<()> {
        let components = component_payload_resolver
            .find_all_components_with_payload_by_version_ids(&document.layout_version_ids).await?;
        
        let raw = components.iter()
            .map(|c| MarkdownRenderService::render_component(&c.payload))
            .collect::<anyhow::Result<Vec<String>>>()?
            .join("\n");

        let published_at = document.finalize_publication()?;

        tokio::fs::write(format!("./exports/doc_{}.md", doc_id), raw).await?;

        document_publisher.update_doc_publication(DocumentPublishParams {
            id: doc_id, 
            status: document.status.to_string(), 
            published_at: Some(published_at),
        }).await?;

        Ok(())
    }
}
