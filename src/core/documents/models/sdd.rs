#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SystemDesignDoc;

impl SystemDesignDoc {
    pub fn is_allowed(&self, component_type: &str) -> bool {
        ["DesignSpec", "Schematic"]
            .iter()
            .any(|prefix| component_type.starts_with(prefix))
    }
}
