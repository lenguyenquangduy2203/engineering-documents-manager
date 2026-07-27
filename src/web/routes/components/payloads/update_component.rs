use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateComponentRequest<T> {
    pub title: String,
    pub payload: T,
}
