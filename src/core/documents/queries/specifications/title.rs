use sqlx::{QueryBuilder, Sqlite};

use crate::infra::dbc::sqlx::FilterSpecification;

/* #region Stateful Service Object */
#[derive(Debug, Clone)]
/* #endregion */
pub struct TitleSpec(pub String);

impl FilterSpecification for TitleSpec {
    fn apply(&self, builder: &mut QueryBuilder<Sqlite>) {
        builder.push(" AND c.current_title LIKE ");
        builder.push_bind(format!("%{}%", self.0));
    }
}
