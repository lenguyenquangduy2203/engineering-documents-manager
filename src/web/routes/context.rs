use std::sync::Arc;

use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct Context {
    dbc: Arc<Pool<Sqlite>>,
}

impl Context {
    pub fn new(dbc: Arc<Pool<Sqlite>>) -> Self {
        Self { dbc }
    }
}
