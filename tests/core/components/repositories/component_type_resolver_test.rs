use std::collections::HashMap;

use engineering_documents_manager::{
    core::components::{
        models::{
            design_specs::{DecisionRecord, DesignSpecSubType}, 
            payload::ComponentPayload::{DesignSpec, Schematic}, 
            schematics::{ImageLink, MermaidDiagram, SchematicSubType}
        }, 
        repositories::component_type_resolver::ComponentTypeResolver
    }, 
    infra::repositories::sqlx::components::SqliteComponentRepository
};

#[path ="../../../utils/mod.rs"]
mod utils;

use utils::sample_provider::SampleProvider;

#[sqlx::test(fixtures(path = "../../../../fixtures", scripts(
    "insert_component_design_spec_decision_record_v1",
    "insert_component_schematic_image_link_v2",
    "insert_component_schematic_mermaid_diagram_v1"
)))]
async fn test_retrieve_all_components_with_type_by_version_ids(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let component_type_repo = setup_component_type_repo(pool);
    let version_ids: [u32; 3] = [1, 3, 4];

    let component_refs = component_type_repo
        .find_all_components_with_type_by_version_ids(&version_ids).await?;

    assert_eq!(component_refs.len(), version_ids.len());

    let component_type_by_version_id: HashMap<_, _> = component_refs.into_iter()
        .map(|rf| (rf.version_id, rf.component_type))
        .collect();
    
    assert_eq!(
        component_type_by_version_id.get(&version_ids[0]),
        Some(&DesignSpec(DesignSpecSubType::DecisionRecord(DecisionRecord::sample())).get_identifier())
    );
    assert_eq!(
        component_type_by_version_id.get(&version_ids[1]),
        Some(&Schematic(SchematicSubType::ImageLink(ImageLink::sample())).get_identifier())
    );
    assert_eq!(
        component_type_by_version_id.get(&version_ids[2]),
        Some(&Schematic(SchematicSubType::MermaidDiagram(MermaidDiagram::sample())).get_identifier())
    );

    anyhow::Ok(())
}

fn setup_component_type_repo(pool: sqlx::SqlitePool) -> Box<dyn ComponentTypeResolver> {
    Box::new(SqliteComponentRepository::new(pool))
}
