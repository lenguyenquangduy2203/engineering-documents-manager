use crate::{
    core::documents::models::doc_status::DocStatus, infra::dbc::sqlx::FilterSpecification,
};

pub struct StatusSpec(pub DocStatus);

impl FilterSpecification for StatusSpec {
    fn apply(&self, builder: &mut sqlx::QueryBuilder<sqlx::Sqlite>) {
        builder.push(" AND d.status = ");
        builder.push_bind(format!("{}", self.0.to_string()));
    }
}
