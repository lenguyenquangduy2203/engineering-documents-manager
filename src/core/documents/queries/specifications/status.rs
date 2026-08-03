use crate::{
    core::documents::models::doc_status::DocStatus, infra::dbc::sqlx::FilterSpecification,
};

/* #region Stateful Service Object */
#[derive(Debug, Clone)]
/* #endregion */
pub struct StatusSpec(pub DocStatus);

impl FilterSpecification for StatusSpec {
    fn apply(&self, builder: &mut sqlx::QueryBuilder<sqlx::Sqlite>) {
        builder.push(" AND d.status = ");
        builder.push_bind(self.0);
    }
}
