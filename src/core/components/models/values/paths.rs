use anyhow::anyhow;

/* #region Value Object */
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/* #endregion */
/* #region Serde DTO */
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(try_from = "String", into = "String")]
/* #endregion */
pub struct ApiPath(pub String);

impl TryFrom<String> for ApiPath {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !value.starts_with('/') {
            return Err(anyhow!("API endpoint must start with a forward slash '/'",));
        }

        Ok(ApiPath(value))
    }
}

impl From<ApiPath> for String {
    fn from(value: ApiPath) -> Self {
        value.0
    }
}

/* #region Value Object */
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/* #endregion */
/* #region Serde DTO */
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(try_from = "String", into = "String")]
/* #endregion */
pub struct AssetPath(pub String);

impl TryFrom<String> for AssetPath {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        // 1. Enforce relative paths starting with '/'
        if !s.starts_with('/') {
            return Err(anyhow!("Asset path must start with a forward slash '/'",));
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
            Err(anyhow!("Asset must point to a valid image extension",))
        }
    }
}

impl From<AssetPath> for String {
    fn from(value: AssetPath) -> Self {
        value.0
    }
}
