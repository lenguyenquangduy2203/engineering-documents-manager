use engineering_documents_manager::core::components::models::{
    design_specs::DecisionRecord,
    references::ApiEndpoint,
    schematics::{ImageLink, MermaidDiagram},
    values::{
        http_method::HttpMethod,
        paths::{ApiPath, AssetPath},
    },
};

pub trait SampleProvider {
    fn sample() -> Self;
}

impl SampleProvider for DecisionRecord {
    fn sample() -> Self {
        Self {
            decision: "ignored".into(),
            rationale: "ignored".into(),
            alternatives_considered: Vec::new(),
        }
    }
}

impl SampleProvider for ImageLink {
    fn sample() -> Self {
        Self {
            path: AssetPath("ignored".into()),
        }
    }
}

impl SampleProvider for MermaidDiagram {
    fn sample() -> Self {
        Self {
            definition: "ignored".into(),
        }
    }
}

impl SampleProvider for ApiEndpoint {
    fn sample() -> Self {
        Self {
            endpoint: ApiPath("ignored".into()),
            method: HttpMethod::Get,
            request_body_example: "ignored".into(),
        }
    }
}
