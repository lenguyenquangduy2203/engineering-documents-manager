use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ApiPath(pub String);

impl<'de> Deserialize<'de> for ApiPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if !s.starts_with('/') {
            return Err(serde::de::Error::custom(
                "API endpoint must start with a forward slash '/'",
            ));
        }

        Ok(ApiPath(s))
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct AssetPath(pub String);

impl<'de> Deserialize<'de> for AssetPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        // 1. Enforce relative paths starting with '/'
        if !s.starts_with('/') {
            return Err(serde::de::Error::custom(
                "Asset path must start with a forward slash '/'",
            ));
        }

        // 2. Validate it's an image file
        let lower = s.to_lowercase();
        if lower.ends_with(".png")
            || lower.ends_with(".jpg")
            || lower.ends_with(".jpeg")
            || lower.ends_with(".svg")
            || lower.ends_with(".webp")
        {
            Ok(AssetPath(s))
        } else {
            Err(serde::de::Error::custom(
                "Asset must point to a valid image extension",
            ))
        }
    }
}
