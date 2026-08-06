use engineering_documents_manager::{
    core::components::{
        models::{
            design_specs::{DecisionRecord, DesignSpecSubType}, 
            payload::ComponentPayload, 
            values::version::Version, 
            wrapper::Component}, 
        repositories::components_repository::{ComponentFilterQuery, ComponentsRepository}
    }, 
    infra::repositories::sqlx::components::SqliteComponentRepository
};

#[sqlx::test]
async fn test_create_new_component(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let component_repo = setup_component_repo(pool);
    let component = create_test_component(None);
    let expected_id = 1;

    let generated_id = component_repo.create_new(&component).await?;
    assert_eq!(generated_id, expected_id);

    anyhow::Ok(())
}

#[sqlx::test]
async fn test_retrieve_component_by_id(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let component_repo = setup_component_repo(pool);
    let component = create_test_component(None);
    let generated_id = component_repo.create_new(&component).await?;

    let saved_component = component_repo
        .find_latest_version_by_id(generated_id).await?
        .expect("Component should exist");
    
    assert_eq!(saved_component.version, component.version);
    assert_eq!(saved_component.payload.get_identifier(), component.payload.get_identifier());
    
    anyhow::Ok(())
}

#[sqlx::test]
async fn test_retrieve_all_components(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let component_repo = setup_component_repo(pool);

    let component_a = create_test_component(None);
    component_repo.create_new(&component_a).await?;
    
    let component_b = create_test_component(None);
    component_repo.create_new(&component_b).await?;
    
    let components = component_repo
        .find_all_latest_version(ComponentFilterQuery::default()).await?;

    assert_eq!(components.len(), 2);

    anyhow::Ok(())
}

#[sqlx::test]
async fn test_update_component(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let component_repo = setup_component_repo(pool);
    let mut component = create_test_component(None);
    let generated_id = component_repo.create_new(&component).await?;

    let updated_title = "Title Updated".to_string();
    let updated_version = component.version.next();
    component.title = updated_title.clone();
    
    let updated_component = component_repo
        .update_component(generated_id, component).await?
        .expect("Component should exist");

    assert_eq!(updated_component.title, updated_title);
    assert_eq!(updated_component.version, updated_version);

    anyhow::Ok(())
}

#[sqlx::test]
async fn test_delete_component(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let component_repo = setup_component_repo(pool);
    let component = create_test_component(None);
    let generated_id = component_repo.create_new(&component).await?;

    let is_component_deleted = component_repo.remove_component_with_all_versions_by_id(generated_id).await?;
    assert!(is_component_deleted);

    let deleted_component = component_repo
        .find_latest_version_by_id(generated_id).await?;
    assert!(deleted_component.is_none());

    anyhow::Ok(())
}

fn setup_component_repo(pool: sqlx::SqlitePool) -> Box<dyn ComponentsRepository> {
    Box::new(SqliteComponentRepository::new(pool))
}

fn create_test_component(with_id: Option<u32>) -> Component<ComponentPayload> {
    Component::<ComponentPayload> { 
        id: with_id, 
        version: Version::default(), 
        title: "Test".into(), 
        payload: ComponentPayload::DesignSpec(DesignSpecSubType::DecisionRecord(DecisionRecord { 
            decision: "Test Decision".into(), 
            rationale: "Test Rationale".into(), 
            alternatives_considered: Vec::new() 
        })) 
    }
}