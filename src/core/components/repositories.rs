use async_trait::async_trait;

use crate::core::components::models::{payload::ComponentPayload, wrapper::Component};

#[async_trait]
pub trait ComponentsRepository: Send + Sync {
    async fn save(&self, component: &Component<ComponentPayload>) -> anyhow::Result<u32>;
}
