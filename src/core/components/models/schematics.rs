use crate::core::components::models::values::paths::AssetPath;

/* #region Domain Entity */
#[derive(Debug, Clone)]
/* #endregion */
/* #region Serde DTO */
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/* #endregion */
pub struct MermaidDiagram {
    pub definition: String,
}

/* #region Domain Entity */
#[derive(Debug, Clone)]
/* #endregion */
/* #region Serde DTO */
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/* #endregion */
pub struct ImageLink {
    pub path: AssetPath,
}

/* #region Domain Entity */
#[derive(Debug, Clone)]
/* #endregion */
/* #region Serde DTO */
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "type", content = "data")]
/* #endregion */
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
