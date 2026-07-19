use crate::{core::documents::models::doc_types::DocTypes, infra::dbc::sqlx::FilterSpecification};

pub struct DocTypeSpec(pub DocTypes);

impl FilterSpecification for DocTypeSpec {
    fn apply(&self, builder: &mut sqlx::QueryBuilder<sqlx::Sqlite>) {
        builder.push(" AND d.type = ");
        builder.push_bind(format!("{}", self.0.to_string()));
    }
}
