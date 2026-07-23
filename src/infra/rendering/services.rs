use anyhow::Ok;

use crate::{
    core::components::models::payload::ComponentPayload, infra::rendering::traits::RenderMarkdown,
};

pub struct MarkdownRenderService;

impl MarkdownRenderService {
    pub fn render_component(payload: &ComponentPayload) -> anyhow::Result<String> {
        let mut out = String::new();
        match payload {
            ComponentPayload::DesignSpec(design_spec_sub_type) => {
                design_spec_sub_type.render_markdown(&mut out)
            }
            ComponentPayload::Schematic(schematic_sub_type) => {
                schematic_sub_type.render_markdown(&mut out)
            }
            ComponentPayload::Reference(reference_sub_type) => {
                reference_sub_type.render_markdown(&mut out)
            }
        };

        Ok(out)
    }
}
