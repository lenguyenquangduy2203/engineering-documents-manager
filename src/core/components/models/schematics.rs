use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct MermaidDiagram {
    pub definition: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ImageLink {
    pub url: String,
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
