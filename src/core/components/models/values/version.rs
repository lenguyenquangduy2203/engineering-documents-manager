use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Version(pub u32);

impl Default for Version {
    fn default() -> Self {
        Version(1)
    }
}
