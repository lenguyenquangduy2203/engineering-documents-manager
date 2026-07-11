use sqlx::{QueryBuilder, Sqlite};

use crate::infra::dbc::sqlx::FilterSpecification;

pub struct SubtypeSpec(pub String);

impl FilterSpecification for SubtypeSpec {
    fn apply(&self, builder: &mut QueryBuilder<Sqlite>) {
        builder.push(" AND c.type LIKE ");
        builder.push_bind(format!("%:{}", self.0));
    }
}
