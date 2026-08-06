use engineering_documents_manager::{core::documents::{models::{doc::Document, doc_types::DocTypes}, repositories::document_lifecycle_manager::{DocumentFilterQuery, DocumentLifecycleManager}}, infra::repositories::sqlx::documents::SqliteDocumentsRepository};

#[sqlx::test]
async fn test_create_new_document(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let document_repo = setup_document_repo(pool);
    let document = create_test_document();
    
    let generated_id = document_repo.create_new(&document).await?;
    
    assert_eq!(generated_id, 1);
    
    anyhow::Ok(())
}

#[sqlx::test]
async fn test_retrieve_document_by_id(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let document_repo = setup_document_repo(pool);
    let document = create_test_document();
    let generated_id = document_repo.create_new(&document).await?;
    
    let saved_document = document_repo
        .find_doc_by_id(generated_id).await?
        .expect("Document should exist");

    assert_eq!(saved_document.doc_type, document.doc_type);
    assert_eq!(saved_document.title, document.title);

    anyhow::Ok(())
}

#[sqlx::test]
async fn test_retrieve_all_documents(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let document_repo = setup_document_repo(pool);

    let document_a = create_test_document();
    document_repo.create_new(&document_a).await?;

    let document_b = create_test_document();
    document_repo.create_new(&document_b).await?;

    let documents = document_repo.find_all_docs(DocumentFilterQuery::default()).await?;

    assert_eq!(documents.len(), 2);

    anyhow::Ok(())
}

#[sqlx::test]
async fn test_delete_document(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let document_repo = setup_document_repo(pool);
    let document = create_test_document();
    let generated_id = document_repo.create_new(&document).await?;

    let is_document_deleted = document_repo.remove_doc_with_all_layouts_by_id(generated_id).await?;
    assert!(is_document_deleted);

    let deleted_document = document_repo.find_doc_by_id(generated_id).await?;
    assert!(deleted_document.is_none());

    anyhow::Ok(())
}

fn setup_document_repo(pool: sqlx::SqlitePool) -> Box<dyn DocumentLifecycleManager> {
    Box::new(SqliteDocumentsRepository::new(pool))
}

fn create_test_document() -> Document {
    Document::new(DocTypes::Sdd, "Test SDD")
}