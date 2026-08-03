use sqlx::{QueryBuilder, Sqlite};

use crate::infra::dbc::sqlx::FilterSpecification;

/* #region Stateful Service Object */
#[derive(Debug, Clone)]
/* #endregion */
pub struct GroupSpec(pub String);

impl FilterSpecification for GroupSpec {
    fn apply(&self, builder: &mut QueryBuilder<Sqlite>) {
        builder.push(" AND c.type LIKE ");
        builder.push_bind(format!("{}:%", self.0));
    }
}
