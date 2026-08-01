use std::ops::Deref;

use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq, Type)]
#[sqlx(transparent)]
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
