use crate::{core::documents::models::doc_types::DocTypes, infra::dbc::sqlx::FilterSpecification};

/* #region Stateful Service Object */
#[derive(Debug, Clone)]
/* #endregion */
pub struct DocTypeSpec(pub DocTypes);

impl FilterSpecification for DocTypeSpec {
    fn apply(&self, builder: &mut sqlx::QueryBuilder<sqlx::Sqlite>) {
        builder.push(" AND d.type = ");
        builder.push_bind(self.0);
    }
}
