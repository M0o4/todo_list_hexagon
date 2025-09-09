use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateInput {
    pub content: String,
    pub completed: bool,
}
