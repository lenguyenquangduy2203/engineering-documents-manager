use engineering_documents_manager::{
    core::components::{
        models::{
            payload::ComponentPayload::{Reference, Schematic}, references::{ApiEndpoint, ReferenceSubType}, schematics::{ImageLink, MermaidDiagram, SchematicSubType}
        }, repositories::component_payload_resolver::ComponentPayloadResolver
    }, infra::repositories::sqlx::components::SqliteComponentRepository
};

#[path ="../../../utils/mod.rs"]
mod utils;

use utils::sample_provider::SampleProvider;

#[sqlx::test(fixtures(path = "../../../../fixtures", scripts("insert_document_draft_api_3_layouts")))]
async fn test_retrieve_all_components_with_payload_by_version_ids(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let component_payload_repo = setup_component_payload_repo(pool);
    let doc_id = 1;
    let version_ids: [u32; 3] = [1, 3, 4];

    let payload_refs = component_payload_repo
        .find_all_components_with_payload_by_version_ids(doc_id, &version_ids).await?;
    assert_eq!(payload_refs.len(), 3);

    assert_eq!(payload_refs[0].component_id, 1);
    assert_eq!(payload_refs[0].version_id, version_ids[0]); // ID: 1
    assert_eq!(
        payload_refs[0].payload.get_identifier(), 
        Schematic(SchematicSubType::MermaidDiagram(MermaidDiagram::sample())).get_identifier()
    );

    assert_eq!(payload_refs[1].component_id, 2);
    assert_eq!(payload_refs[1].version_id, version_ids[1]); // ID: 3 (v2)
    assert_eq!(
        payload_refs[1].payload.get_identifier(),
        Schematic(SchematicSubType::ImageLink(ImageLink::sample())).get_identifier()
    );

    assert_eq!(payload_refs[2].component_id, 3);
    assert_eq!(payload_refs[2].version_id, version_ids[2]); // ID: 4
    assert_eq!(
        payload_refs[2].payload.get_identifier(),
        Reference(ReferenceSubType::ApiEndpoint(ApiEndpoint::sample())).get_identifier()
    );

    anyhow::Ok(())
}

fn setup_component_payload_repo(pool: sqlx::SqlitePool) -> Box<dyn ComponentPayloadResolver> {
    Box::new(SqliteComponentRepository::new(pool))
}
