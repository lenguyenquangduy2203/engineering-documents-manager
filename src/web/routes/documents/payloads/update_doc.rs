use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateDocRequest {
    pub title: Option<String>,
}
