/* #region Service Object */
#[derive(Debug, Clone, Copy, Default)]
/* #endregion */
pub struct SystemDesignDoc;

impl SystemDesignDoc {
    pub fn is_allowed(component_type: &str) -> bool {
        ["DesignSpec", "Schematic"]
            .iter()
            .any(|prefix| component_type.starts_with(prefix))
    }
}
