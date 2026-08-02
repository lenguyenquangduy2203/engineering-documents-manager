use std::ops::Deref;

/* #region Value Object */
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/* #endregion */
/* #region Serde DTO */
#[derive(serde::Serialize, serde::Deserialize)]
/* #endregion */
pub struct LayoutVersionIds(pub Vec<u32>);

// Allow easy access to the underlying Vec<u32> (e.g., ids.len(), ids.iter())
impl Deref for LayoutVersionIds {
    type Target = Vec<u32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Option<String>> for LayoutVersionIds {
    fn from(opt: Option<String>) -> Self {
        let ids = match opt {
            Some(s) if !s.trim().is_empty() => s
                .split(',')
                .filter_map(|val| val.trim().parse::<u32>().ok())
                .collect(),
            _ => Vec::new(),
        };

        LayoutVersionIds(ids)
    }
}
