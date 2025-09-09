use domain::models::task::TaskId;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreateOutput {
    pub id: TaskId,
}
