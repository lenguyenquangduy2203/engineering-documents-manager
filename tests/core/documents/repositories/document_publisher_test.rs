use engineering_documents_manager::{
    core::documents::{
        models::doc_status::DocStatus,
        repositories::document_publisher::{DocumentPublishParams, DocumentPublisher},
    },
    infra::repositories::sqlx::documents::SqliteDocumentsRepository,
};

#[sqlx::test(fixtures(
    path = "../../../../fixtures",
    scripts("insert_document_draft_api_3_layouts")
))]
fn test_publish_document(pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    let document_repo = setup_publish_repo(pool.clone());
    let published_at = chrono::Utc::now();
    let params = DocumentPublishParams {
        id: 1,
        status: DocStatus::Published.to_string(),
        published_at: Some(published_at),
    };

    document_repo.update_doc_publication(params.clone()).await?;
    let query: sqlx::query::QueryAs<'_, sqlx::Sqlite, UpdatedColumns, _> = sqlx::query_as(
        r#"
            SELECT status, is_completed, published_at
            FROM documents
            WHERE id = ?
        "#,
    )
    .bind(params.id);
    let updated_columns = query.fetch_one(&pool).await?;

    assert_eq!(updated_columns.status, params.status);
    assert!(updated_columns.is_completed);
    assert_eq!(
        updated_columns.published_at,
        published_at.format("%Y-%m-%d %H:%M:%S").to_string()
    );

    anyhow::Ok(())
}

fn setup_publish_repo(pool: sqlx::SqlitePool) -> Box<dyn DocumentPublisher> {
    Box::new(SqliteDocumentsRepository::new(pool))
}

/* #region Sqlx Record */
#[derive(sqlx::FromRow)]
/* #endregion */
struct UpdatedColumns {
    pub status: String,
    pub is_completed: bool,
    pub published_at: String,
}
