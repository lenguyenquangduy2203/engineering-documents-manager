use async_trait::async_trait;
use serde::Serialize;

use crate::core::components::models::Component;

#[async_trait]
pub trait ComponentsRepository: Send + Sync {
    async fn save<T>(&self, component: &Component<T>) -> anyhow::Result<u32>
    where T: Serialize + Send + Sync;
}
