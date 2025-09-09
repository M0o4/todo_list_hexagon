use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateInput {
    pub content: String,
}
