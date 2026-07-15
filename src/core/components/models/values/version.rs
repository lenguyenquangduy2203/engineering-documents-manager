use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
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
