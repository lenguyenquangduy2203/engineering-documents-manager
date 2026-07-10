use serde::{Deserialize, Serialize};

use crate::core::components::models::values::paths::AssetPath;

#[derive(Deserialize, Serialize, Clone)]
pub struct MermaidDiagram {
    pub definition: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ImageLink {
    pub path: AssetPath,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum SchematicSubType {
    MermaidDiagram(MermaidDiagram),
    ImageLink(ImageLink),
}

impl SchematicSubType {
    pub fn get_type(&self) -> String {
        match self {
            SchematicSubType::MermaidDiagram(_) => "MermaidDiagram".into(),
            SchematicSubType::ImageLink(_) => "ImageLink".into(),
        }
    }
}
