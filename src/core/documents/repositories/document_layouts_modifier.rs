use async_trait::async_trait;

#[async_trait]
pub trait DocumentLayoutsModifier: Send + Sync {
    async fn replace_layouts(&self, doc_id: u32, version_ids: &[u32]) -> anyhow::Result<()>;
}
