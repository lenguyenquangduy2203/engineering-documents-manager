use std::sync::Arc;

use axum_macros::FromRef;

use crate::core::components::repositories::ComponentsRepository;

#[derive(Clone, FromRef)]
pub struct Context {
    component_repository: Arc<dyn ComponentsRepository>,
}

impl Context {
    pub fn new(component_repository: Arc<dyn ComponentsRepository>) -> Self {
        Self {
            component_repository,
        }
    }
}
