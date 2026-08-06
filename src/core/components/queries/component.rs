use crate::{
    core::components::{
        queries::specifications::{group::GroupSpec, sub_type::SubtypeSpec, title::TitleSpec},
        repositories::components_repository::ComponentFilterQuery,
    },
    infra::dbc::sqlx::FilterSpecification,
};

/* #region Builder */
#[derive(Debug, Default)]
/* #endregion */
pub struct ComponentQuery {
    specs: Vec<Box<dyn FilterSpecification>>,
    pub limit: i64,
    pub offset: i64,
}

impl ComponentQuery {
    /// Evaluates the incoming API Request Query and dynamically aggregates valid criteria
    pub fn new(filter: ComponentFilterQuery) -> Self {
        let mut specs: Vec<Box<dyn FilterSpecification>> = Vec::new();

        if let Some(title) = filter.title {
            specs.push(Box::new(TitleSpec(title)));
        }

        if let Some(group) = filter.group {
            specs.push(Box::new(GroupSpec(group)));
        }

        if let Some(subtype) = filter.subtype {
            specs.push(Box::new(SubtypeSpec(subtype)));
        }

        Self {
            specs,
            limit: filter.limit.unwrap_or(20),
            offset: filter.offset.unwrap_or(0),
        }
    }

    /// The pipeline mechanism that allows the query to construct itself
    pub fn apply(&self, builder: &mut sqlx::QueryBuilder<sqlx::Sqlite>) {
        // Map-apply every specification item seamlessly
        for spec in &self.specs {
            spec.apply(builder);
        }

        // Apply global pagination bounds
        builder.push(" LIMIT ");
        builder.push_bind(self.limit);
        builder.push(" OFFSET ");
        builder.push_bind(self.offset);
    }
}
