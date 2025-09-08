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

impl Task {
    pub fn new(task_id: TaskId, text: String, completed: bool, create_at: Datetime) -> Self {
        Self {
            text,
            task_id,
            completed,
            create_at,
        }
    }
}
