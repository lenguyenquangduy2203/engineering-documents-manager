use anyhow::anyhow;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::core::documents::models::{api::ApiDoc, sdd::SystemDesignDoc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocTypes {
    SDD(SystemDesignDoc),
    API(ApiDoc),
}

impl DocTypes {
    pub fn is_allowed(&self, component_type: &str) -> bool {
        match self {
            DocTypes::SDD(system_design_doc) => system_design_doc.is_allowed(component_type),
            DocTypes::API(api_doc) => api_doc.is_allowed(component_type),
        }
    }
}

impl TryFrom<&str> for DocTypes {
    type Error = anyhow::Error;

    fn try_from(doc_type: &str) -> Result<Self, Self::Error> {
        match doc_type {
            "SDD" => Ok(DocTypes::SDD(SystemDesignDoc)),
            "API" => Ok(DocTypes::API(ApiDoc)),
            _ => Err(anyhow!("Unknown document type: {}", doc_type)),
        }
    }
}

// Custom Serializer: Converts the enum variant directly to a flat string
impl Serialize for DocTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DocTypes::SDD(_) => serializer.serialize_str("SDD"),
            DocTypes::API(_) => serializer.serialize_str("API"),
        }
    }
}

// Custom Deserializer: Allows Serde to read a flat string back into the correct variant
impl<'de> Deserialize<'de> for DocTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DocTypes::try_from(s.as_str()).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}
