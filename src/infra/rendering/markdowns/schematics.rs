use std::fmt::Write;

use crate::{
    core::components::models::schematics::{ImageLink, MermaidDiagram, SchematicSubType},
    infra::rendering::traits::RenderMarkdown,
};

impl RenderMarkdown for MermaidDiagram {
    fn render_markdown(&self, out: &mut String) {
        writeln!(out, "```mermaid").unwrap();
        writeln!(out, "{}", self.definition.trim()).unwrap();
        writeln!(out, "```\n").unwrap();
    }
}

impl RenderMarkdown for ImageLink {
    fn render_markdown(&self, out: &mut String) {
        writeln!(out, "![Asset]({})\n", self.path.0).unwrap();
    }
}

impl RenderMarkdown for SchematicSubType {
    fn render_markdown(&self, out: &mut String) {
        match self {
            SchematicSubType::MermaidDiagram(mermaid_diagram) => {
                mermaid_diagram.render_markdown(out)
            }
            SchematicSubType::ImageLink(image_link) => image_link.render_markdown(out),
        }
    }
}
