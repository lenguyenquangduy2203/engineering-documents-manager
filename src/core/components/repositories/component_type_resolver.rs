use async_trait::async_trait;

pub struct ComponentRef {
    pub id: u32,
    pub version_id: u32,
    pub component_type: String,
}

#[async_trait]
pub trait ComponentTypeResolver: Send + Sync {
    async fn find_all_components_with_type_by_version_ids(
        &self,
        version_ids: &[u32]
    ) -> anyhow::Result<Vec<ComponentRef>>;
}