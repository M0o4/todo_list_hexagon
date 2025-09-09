use crate::models::task::TaskId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
    task_id: TaskId,
    content: String,
    completed: bool,
}

impl Task {
    pub fn new(task_id: TaskId, content: String, completed: bool) -> Self {
        Self {
            content,
            completed,
            task_id,
        }
    }

    pub fn id_bytes(&self) -> [u8; 16] {
        self.task_id.0.to_bytes()
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn completed(&self) -> bool {
        self.completed
    }
}
