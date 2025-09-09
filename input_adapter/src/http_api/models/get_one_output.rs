use domain::models::task::TaskId;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetOneOutput {
    pub id: TaskId,
    pub content: String,
    pub completed: bool,
    pub created_at: String,
}
