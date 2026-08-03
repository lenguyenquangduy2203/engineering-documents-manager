use crate::core::components::models::{
    design_specs::DesignSpecSubType, references::ReferenceSubType, schematics::SchematicSubType,
};

/* #region Domain Entity */
#[derive(Debug, Clone)]
/* #endregion */
/* #region Serde DTO */
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "group", content = "data")]
/* #endregion */
pub enum ComponentPayload {
    DesignSpec(DesignSpecSubType),
    Schematic(SchematicSubType),
    Reference(ReferenceSubType),
}

impl ComponentPayload {
    pub fn get_group(&self) -> String {
        match self {
            ComponentPayload::DesignSpec(_) => "DesignSpec".into(),
            ComponentPayload::Schematic(_) => "Schematic".into(),
            ComponentPayload::Reference(_) => "Reference".into(),
        }
    }

    pub fn get_identifier(&self) -> String {
        match self {
            ComponentPayload::DesignSpec(design_spec_sub_type) => {
                format!("{}:{}", self.get_group(), design_spec_sub_type.get_type())
            }
            ComponentPayload::Schematic(schematic_sub_type) => {
                format!("{}:{}", self.get_group(), schematic_sub_type.get_type())
            }
            ComponentPayload::Reference(reference_sub_type) => {
                format!("{}:{}", self.get_group(), reference_sub_type.get_type())
            }
        }
    }
}
