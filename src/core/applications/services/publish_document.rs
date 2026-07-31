use std::sync::Arc;

use crate::{
    core::{
        applications::errors::publish_doc::PublishDocumentError, 
        components::{models::payload::ComponentPayload, repositories::ComponentPayloadResolver}, 
        documents::{
            models::doc::Document, 
            repositories::{DocumentPublishParams, DocumentPublisher, DocumentsResolver}            
        },
    }, infra::rendering::services::DocumentExportService
};

pub struct DocumentPublishingService;

type DocumentPublishingServiceDepsTuple = (
    Arc<dyn DocumentsResolver>,
    Arc<dyn ComponentPayloadResolver>,
    Arc<dyn DocumentPublisher>,
    Arc<dyn DocumentExportService>,
);

impl DocumentPublishingService {
    pub async fn publish_document(
        (
            documents_resolver, component_payload_resolver, 
            document_publisher, document_export_service
        ): DocumentPublishingServiceDepsTuple,
        doc_id: u32,
    ) -> std::result::Result<(), PublishDocumentError> {
        let mut document = documents_resolver.find_doc_by_id(doc_id).await?
            .ok_or(PublishDocumentError::NotFound(doc_id))?;

        document.marked_for_publishing()?;

        document_publisher.update_doc_publication(DocumentPublishParams { 
            id: doc_id, 
            status: document.status.to_string(), 
            published_at: None, 
        }).await?;

        let component_payload_resolver = Arc::clone(&component_payload_resolver);
        let document_publisher = Arc::clone(&document_publisher);
        let document_export_service = Arc::clone(&document_export_service);

        tokio::spawn(async move {
            match Self::execute_publish_task(
                component_payload_resolver.as_ref(),
                document_publisher.as_ref(),
                document_export_service.as_ref(), 
                doc_id,
                &mut document
            ).await {
                Ok(_) => {
                    tracing::info!(doc_id = doc_id, "Successfully published document & saved file");
                }
                Err(err) => {
                    tracing::error!(target: "publishing", error = %err, doc_id = doc_id, "Background publish failed");
                    if let Err(transition_err) = document.mark_failed() {
                        tracing::error!(
                            doc_id = doc_id, 
                            error = %transition_err, 
                            "Failed to transition document state to Failed"
                        );

                        return; // Prevent updating DB with an invalid state
                    }
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
        export_service: &dyn DocumentExportService,
        doc_id: u32,
        document: &mut Document,
    ) -> anyhow::Result<()> {
        let payloads: Vec<ComponentPayload> = component_payload_resolver
            .find_all_components_with_payload_by_version_ids(doc_id, &document.layout_version_ids).await?
            .into_iter()
            .map(|c| c.payload)
            .collect();
        
        let published_at = document.finalize_publication()?;

        export_service.render_and_save(document, &payloads).await?;

        document_publisher.update_doc_publication(DocumentPublishParams {
            id: doc_id, 
            status: document.status.to_string(), 
            published_at: Some(published_at),
        }).await?;

        Ok(())
    }
}
