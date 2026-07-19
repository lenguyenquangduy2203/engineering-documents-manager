use crate::{
    core::documents::{
        queries::specifications::{doc_type::DocTypeSpec, status::StatusSpec, title::TitleSpec},
        repositories::DocumentFilterQuery,
    },
    infra::dbc::sqlx::FilterSpecification,
};

pub struct DocumentQuery {
    specs: Vec<Box<dyn FilterSpecification>>,
    pub limit: i64,
    pub offset: i64,
}

impl DocumentQuery {
    pub fn new(filter: DocumentFilterQuery) -> Self {
        let mut specs: Vec<Box<dyn FilterSpecification>> = Vec::new();

        if let Some(title) = filter.title {
            specs.push(Box::new(TitleSpec(title)));
        }

        if let Some(status) = filter.status {
            specs.push(Box::new(StatusSpec(status)));
        }

        if let Some(doc_type) = filter.doc_type {
            specs.push(Box::new(DocTypeSpec(doc_type)));
        }

        Self {
            specs,
            limit: filter.limit.unwrap_or(20),
            offset: filter.offset.unwrap_or(0),
        }
    }

    pub fn apply(&self, builder: &mut sqlx::QueryBuilder<sqlx::Sqlite>) {
        for spec in &self.specs {
            spec.apply(builder);
        }

        builder.push(" LIMIT ");
        builder.push_bind(self.limit);
        builder.push(" OFFSET ");
        builder.push_bind(self.offset);
    }
}
