use crate::models::task::TaskId;
use crate::models::utils::Datetime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
    task_id: TaskId,
    text: String,
    completed: bool,
    create_at: Datetime,
}
