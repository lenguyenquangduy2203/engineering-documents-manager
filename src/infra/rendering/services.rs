use std::path::{Path, PathBuf};

use anyhow::{Context, Ok};
use async_trait::async_trait;
use tokio::fs;

use crate::{
    core::{components::models::payload::ComponentPayload, documents::models::doc::Document}, infra::rendering::traits::RenderMarkdown,
};

#[async_trait]
pub trait DocumentExportService: Send + Sync {
    async fn render_and_save(
        &self,
        document: &Document,
        payloads: &[ComponentPayload],
    ) -> anyhow::Result<PathBuf>;
}

pub struct MarkdownRenderService {
    output_dir: PathBuf,
}

impl MarkdownRenderService {
    pub fn new<P: AsRef<Path>>(output_dir: P) -> Self {
        Self {
            output_dir: output_dir.as_ref().to_path_buf(),
        }
    }
    
    /// Pure helper to render an individual component payload
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

    /// Assembles full Markdown content (Metadata Frontmatter + Title + Body)
    pub fn render_document(&self, document: &Document, payloads: &[ComponentPayload]) -> anyhow::Result<String> {
        let mut out = String::new();

        // 1. YAML Frontmatter for Metadata
        out.push_str("---\n");
        if let Some(id) = document.id {
            out.push_str(&format!("id: {}\n", id));
        }
        out.push_str(&format!("document type: {:?}\n", document.doc_type));
        out.push_str(&format!("status: {:?}\n", document.status));
        out.push_str("---\n\n");

        // 2. Document Title
        out.push_str(&format!("# {}\n\n", document.title));

        // 3. Component Contents
        for payload in payloads {
            let rendered = Self::render_component(payload)?;
            out.push_str(&rendered);
            out.push_str("\n\n");
        }

        Ok(out.trim_end().to_string())
    }
}

#[async_trait]
impl DocumentExportService for MarkdownRenderService {
    async fn render_and_save(
        &self,
        document: &Document,
        payloads: &[ComponentPayload],
    ) -> anyhow::Result<PathBuf> {
        let content = self.render_document(document, payloads)?;
        
        let doc_id = document.id.unwrap_or(0);
        let file_path = self.output_dir.join(format!("doc_{}.md", doc_id));

        // Ensure target directory exists before writing
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)
                .await
                .with_context(|| format!("Failed to create output directory: {:?}", parent.display()))?;
        }

        fs::write(&file_path, content)
            .await
            .with_context(|| format!("Failed to write document to {:?}", file_path.display()))?;

        Ok(file_path)
    }
}