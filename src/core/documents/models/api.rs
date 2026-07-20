#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ApiDoc;

impl ApiDoc {
    pub fn is_allowed(&self, component_type: &str) -> bool {
        ["Schematic", "Reference"]
            .iter()
            .any(|prefix| component_type.starts_with(prefix))
    }
}
