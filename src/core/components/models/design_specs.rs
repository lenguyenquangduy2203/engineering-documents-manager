/* #region Domain Entity */
#[derive(Debug, Clone)]
/* #endregion */
/* #region Serde DTO */
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/* #endregion */
pub struct DecisionRecord {
    pub decision: String,
    pub rationale: String,
    pub alternatives_considered: Vec<String>,
}

/* #region Domain Entity */
#[derive(Debug, Clone)]
/* #endregion */
/* #region Serde DTO */
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "type", content = "data")]
/* #endregion */
pub enum DesignSpecSubType {
    DecisionRecord(DecisionRecord),
}

impl DesignSpecSubType {
    pub fn get_type(&self) -> String {
        match self {
            DesignSpecSubType::DecisionRecord(_) => "DecisionRecord".into(),
        }
    }
}
