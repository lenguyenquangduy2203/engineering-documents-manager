use engineering_documents_manager::core::components::models::{
    design_specs::DecisionRecord,
    schematics::{ImageLink, MermaidDiagram},
    values::paths::AssetPath,
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
