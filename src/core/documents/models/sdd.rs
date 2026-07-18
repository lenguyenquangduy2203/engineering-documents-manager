#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SystemDesignDoc;

impl SystemDesignDoc {
    pub fn is_allowed(&self, component_type: &str) -> bool {
        matches!(component_type, "DesignSpec" | "Schematic")
    }
}
