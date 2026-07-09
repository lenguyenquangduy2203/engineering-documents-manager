use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait ComponentsRepository: Send + Sync {
    async fn save(&self, component_type: &str, title: &str, payload: Value) -> anyhow::Result<u32>;
}
