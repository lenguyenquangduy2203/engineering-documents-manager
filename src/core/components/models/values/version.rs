use std::ops::Deref;

/* #region Tiny Value Object */
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/* #endregion */
/* #region Serde DTO */
#[derive(serde::Serialize, serde::Deserialize)]
/* #endregion */
/* #region Sqlx Data Type */
#[derive(sqlx::Type)]
#[sqlx(transparent)]
/* #endregion */
pub struct Version(pub u32);

impl Version {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl Default for Version {
    fn default() -> Self {
        Version(1)
    }
}

impl Deref for Version {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u32> for Version {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
