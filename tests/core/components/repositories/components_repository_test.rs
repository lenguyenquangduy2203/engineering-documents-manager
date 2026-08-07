use engineering_documents_manager::{
    core::components::{
        models::{
            design_specs::{DecisionRecord, DesignSpecSubType}, payload::ComponentPayload::{self, DesignSpec, Schematic}, references::{ApiEndpoint, ReferenceSubType}, schematics::{ImageLink, MermaidDiagram, SchematicSubType}, values::{http_method::HttpMethod, paths::{ApiPath, AssetPath}, version::Version}, wrapper::Component}, repositories::components_repository::{ComponentFilterQuery, ComponentsRepository}
    }, infra::repositories::sqlx::components::SqliteComponentRepository
};

#[path ="../../../utils/mod.rs"]
mod utils;

use utils::sample_provider::SampleProvider;

#[sqlx::test]
async fn test_create_new_component(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let component_repo = setup_component_repo(pool);
    let expected_id = 1;
    let component = Component::<ComponentPayload> { 
        id: None, 
        version: Version::default(), 
        title: "Test".into(), 
        payload: ComponentPayload::DesignSpec(DesignSpecSubType::DecisionRecord(DecisionRecord { 
            decision: "Test Decision".into(), 
            rationale: "Test Rationale".into(), 
            alternatives_considered: Vec::new() 
        })) 
    };

    let generated_id = component_repo.create_new(&component).await?;
    assert_eq!(generated_id, expected_id);

    anyhow::Ok(())
}

#[sqlx::test(fixtures(path = "../../../../fixtures", scripts("insert_component_schematic_image_link_v2")))]
async fn test_retrieve_component_by_id(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let component_repo = setup_component_repo(pool);
    
    let wanted_id = 1;
    let expected_version = Version::default().next();
    let expected_payload = ComponentPayload::Schematic(SchematicSubType::ImageLink(
        ImageLink { path: AssetPath("ignored".into()) }
    ));

    let component = component_repo
        .find_latest_version_by_id(wanted_id).await?
        .expect("Component should exist");
    
    assert_eq!(component.version, expected_version);
    assert_eq!(component.payload.get_identifier(), expected_payload.get_identifier());
    
    anyhow::Ok(())
}

#[sqlx::test(fixtures(path = "../../../../fixtures", scripts(
    "insert_component_design_spec_decision_record_v1",
    "insert_component_schematic_image_link_v2",
    "insert_component_schematic_mermaid_diagram_v1"
)))]
async fn test_retrieve_all_components(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let component_repo = setup_component_repo(pool);

    let components = component_repo
        .find_all_latest_version(ComponentFilterQuery::default())
        .await?;

    assert_eq!(components.len(), 3);

    // 1. Assert DesignSpec Decision Record
    let decision_record = components
        .get(0)
        .expect("Expected design spec decision record at index 0");
    assert_eq!(decision_record.version, Version::default());
    assert_eq!(
        decision_record.payload.get_identifier(),
        DesignSpec(DesignSpecSubType::DecisionRecord(DecisionRecord::sample())).get_identifier()
    );

    // 2. Assert Schematic Image Link
    let expected_image_link_version = Version::default().next();
    let image_link = components
        .get(1)
        .expect("Expected schematic image link at index 1");
    assert_eq!(image_link.version, expected_image_link_version);
    assert_eq!(
        image_link.payload.get_identifier(),
        Schematic(SchematicSubType::ImageLink(ImageLink::sample())).get_identifier()
    );

    // 3. Assert Schematic Mermaid Diagram
    let mermaid_diagram = components
        .get(2)
        .expect("Expected schematic mermaid diagram at index 2");
    assert_eq!(mermaid_diagram.version, Version::default());
    assert_eq!(
        mermaid_diagram.payload.get_identifier(),
        Schematic(SchematicSubType::MermaidDiagram(MermaidDiagram::sample())).get_identifier()
    );

    anyhow::Ok(())
}

#[sqlx::test(fixtures(path = "../../../../fixtures", scripts("insert_component_reference_api_endpoint_v1")))]
async fn test_update_component(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let component_repo = setup_component_repo(pool);

    let expected_updated_title = "Title Updated";
    let expected_updated_version = Version::default().next();
    let incoming_api_ref = Component { 
        id: Some(1), 
        version: Version::default(), 
        title: expected_updated_title.to_string(), 
        payload: ComponentPayload::Reference(ReferenceSubType::ApiEndpoint(
            ApiEndpoint { 
                endpoint: ApiPath("/api/v2/users/register".to_string()), 
                method: HttpMethod::Post, 
                request_body_example: "{\n  \"email\": \"dev@example.com\",\n  \"password\": \"Secret123!\"\n}".to_string() }
        )) };
    
    let updated_component = component_repo
        .update_component(1, incoming_api_ref).await?
        .expect("Component should exist");

    assert_eq!(updated_component.title, expected_updated_title.to_string());
    assert_eq!(updated_component.version, expected_updated_version);

    anyhow::Ok(())
}

#[sqlx::test(fixtures(path = "../../../../fixtures", scripts("insert_component_design_spec_decision_record_v1")))]
async fn test_delete_component(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let component_repo = setup_component_repo(pool.clone());

    let is_component_deleted = component_repo.remove_component_with_all_versions_by_id(1).await?;
    assert!(is_component_deleted);

    let deleted_component = component_repo
        .find_latest_version_by_id(1).await?;
    assert!(deleted_component.is_none());

    let remaining_versions: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM component_versions WHERE component_id = ?")
        .bind(1)
        .fetch_one(&pool)
        .await?;
    assert_eq!(remaining_versions, 0, "Component versions were not cascaded during deletion");

    anyhow::Ok(())
}

fn setup_component_repo(pool: sqlx::SqlitePool) -> Box<dyn ComponentsRepository> {
    Box::new(SqliteComponentRepository::new(pool))
}
