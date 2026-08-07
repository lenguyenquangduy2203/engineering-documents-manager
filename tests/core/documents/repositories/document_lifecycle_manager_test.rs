use engineering_documents_manager::{core::documents::{models::{doc::Document, doc_status::DocStatus, doc_types::DocTypes}, repositories::document_lifecycle_manager::{DocumentFilterQuery, DocumentLifecycleManager}}, infra::repositories::sqlx::documents::SqliteDocumentsRepository};

#[sqlx::test]
async fn test_create_new_document(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let document_repo = setup_document_repo(pool);
    let expected_id = 1;
    let document = Document::new(DocTypes::Sdd, "Test SDD");

    let generated_id = document_repo.create_new(&document).await?;
    assert_eq!(generated_id, expected_id);
    
    anyhow::Ok(())
}

#[sqlx::test(fixtures(path = "../../../../fixtures", scripts("insert_document_draft_sdd_empty_layouts")))]
async fn test_retrieve_document_by_id(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let document_repo = setup_document_repo(pool);
    let wanted_id = 1;
    let expected_doc_type = DocTypes::Sdd;
    let expected_doc_status = DocStatus::Draft;
    
    let document = document_repo
        .find_doc_by_id(wanted_id).await?
        .expect("Document should exist");

    assert_eq!(document.doc_type, expected_doc_type);
    assert_eq!(document.status, expected_doc_status);
    assert!(document.layout_version_ids.is_empty());

    anyhow::Ok(())
}

#[sqlx::test(fixtures(path = "../../../../fixtures", scripts(
    "insert_document_draft_sdd_empty_layouts",
    "insert_document_draft_api_3_layouts"
)))]
async fn test_retrieve_all_documents(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let document_repo = setup_document_repo(pool);
    let expected_len = 2;
    let expected_api_doc_layouts_number = 3;

    let documents = document_repo.find_all_docs(DocumentFilterQuery::default()).await?;
    assert_eq!(documents.len(), expected_len);

    let api_doc = documents.get(1).expect("Expected at least 2 documents returned from DB");
    assert_eq!(api_doc.layout_version_ids.len(), expected_api_doc_layouts_number);

    anyhow::Ok(())
}

#[sqlx::test(fixtures(path = "../../../../fixtures", scripts("insert_document_draft_api_3_layouts")))]
async fn test_delete_document(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let document_repo = setup_document_repo(pool.clone());

    let is_document_deleted = document_repo.remove_doc_with_all_layouts_by_id(1).await?;
    assert!(is_document_deleted);

    let deleted_document = document_repo.find_doc_by_id(1).await?;
    assert!(deleted_document.is_none());

    let remaining_layouts: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM document_layouts WHERE document_id = ?")
        .bind(1)
        .fetch_one(&pool)
        .await?;
    assert_eq!(remaining_layouts, 0, "Document layouts were not cascaded during deletion");

    anyhow::Ok(())
}

fn setup_document_repo(pool: sqlx::SqlitePool) -> Box<dyn DocumentLifecycleManager> {
    Box::new(SqliteDocumentsRepository::new(pool))
}
