use crate::models::task::TaskId;
use crate::models::utils::Datetime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
    task_id: TaskId,
    content: String,
    completed: bool,
    created_at: Datetime,
}

impl Task {
    pub fn new(task_id: TaskId, content: String, completed: bool, created_at: Datetime) -> Self {
        Self {
            content,
            task_id,
            completed,
            created_at,
        }
    }

    pub fn task_id(&self) -> &TaskId {
        &self.task_id
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub fn created_at(&self) -> &Datetime {
        &self.created_at
    }
}
