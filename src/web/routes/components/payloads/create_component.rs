use crate::core::components::models::{values::version::Version, wrapper::Component};

/* #region Serde Request */
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
/* #endregion */
pub struct CreateComponentRequest<T> {
    pub title: String,
    pub payload: T,
}

impl<T> From<CreateComponentRequest<T>> for Component<T> {
    fn from(req: CreateComponentRequest<T>) -> Self {
        Component {
            id: None,
            version: Version::default(),
            title: req.title,
            payload: req.payload,
        }
    }
}

/* #region Serde Response */
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
/* #endregion */
pub struct CreatedResponse {
    pub id: u32,
}
