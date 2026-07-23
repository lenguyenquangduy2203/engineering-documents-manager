use std::fmt::Write;

use crate::{
    core::components::models::references::{ApiEndpoint, ReferenceSubType},
    infra::rendering::traits::RenderMarkdown,
};

impl RenderMarkdown for ApiEndpoint {
    fn render_markdown(&self, out: &mut String) {
        writeln!(out, "### `{}` `{}`\n", self.method, self.endpoint.0).unwrap();
        if !self.request_body_example.is_empty() {
            writeln!(out, "**Request Body Example:**\n").unwrap();
            writeln!(out, "```json").unwrap();
            writeln!(out, "{}", self.request_body_example.trim()).unwrap();
            writeln!(out, "```\n").unwrap();
        }
    }
}

impl RenderMarkdown for ReferenceSubType {
    fn render_markdown(&self, out: &mut String) {
        match self {
            ReferenceSubType::ApiEndpoint(api_endpoint) => api_endpoint.render_markdown(out),
        }
    }
}
