use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DecisionRecord {
    pub decision: String,
    pub rationale: String,
    pub alternatives_considered: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
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
