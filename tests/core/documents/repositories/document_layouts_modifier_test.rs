use engineering_documents_manager::{
    core::documents::{models::layout_version_ids::LayoutVersionIds, repositories::{
        document_layouts_modifier::DocumentLayoutsModifier, document_resolver::DocumentsResolver
    }}, infra::repositories::sqlx::documents::SqliteDocumentsRepository
};

#[sqlx::test(fixtures(path = "../../../../fixtures", scripts("insert_document_draft_api_3_layouts")))]
async fn test_replace_document_layouts(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let (modifier, resolver) = setup_ctx(pool);
    
    let doc_id = 1;
    let reversed_version_ids: [u32; 3] = [4, 3, 1];
    let expected_layout_version_ids = LayoutVersionIds::from(Some(String::from("4,3,1")));

    modifier.replace_layouts(doc_id, &reversed_version_ids).await?;
    let updated_doc = resolver.find_doc_by_id(doc_id).await?.expect("Document should exist");

    assert_eq!(updated_doc.layout_version_ids, expected_layout_version_ids);

    anyhow::Ok(())
}

fn setup_ctx(pool: sqlx::SqlitePool) -> (Box<dyn DocumentLayoutsModifier>, Box<dyn DocumentsResolver>) {
    let repo = SqliteDocumentsRepository::new(pool);

    (Box::new(repo.clone()), Box::new(repo))
}
