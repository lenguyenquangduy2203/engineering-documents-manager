#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ApiDoc;

impl ApiDoc {
    pub fn is_allowed(&self, component_type: &str) -> bool {
        matches!(component_type, "Schematic" | "Reference")
    }
}
