use std::fmt::Write;

use crate::{
    core::components::models::design_specs::{DecisionRecord, DesignSpecSubType},
    infra::rendering::traits::RenderMarkdown,
};

impl RenderMarkdown for DecisionRecord {
    fn render_markdown(&self, out: &mut String) {
        writeln!(out, "### Decision\n{}\n", self.decision).unwrap();
        writeln!(out, "### Rationale\n{}\n", self.rationale).unwrap();
        if !self.alternatives_considered.is_empty() {
            writeln!(out, "### Alternatives Considered").unwrap();
            for alt in &self.alternatives_considered {
                writeln!(out, "- {alt}").unwrap();
            }
        }
    }
}

impl RenderMarkdown for DesignSpecSubType {
    fn render_markdown(&self, out: &mut String) {
        match self {
            DesignSpecSubType::DecisionRecord(decision_record) => {
                decision_record.render_markdown(out)
            }
        }
    }
}
