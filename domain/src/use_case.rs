use crate::app_errors::AppError;
use crate::input_ports::task_command::TaskCommand;
use crate::input_ports::task_query::TaskQuery;
use crate::models::task::TaskId;
use crate::models::task_command::Task;
use crate::output_ports::{task_command, task_query};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TaskUseCase<A, B>
where
    A: task_command::TaskCommand,
    B: task_query::TaskQuery,
{
    task_command: Arc<A>,
    task_query: Arc<B>,
}

impl<A, B> TaskUseCase<A, B>
where
    A: task_command::TaskCommand,
    B: task_query::TaskQuery,
{
    pub fn new(task_command: Arc<A>, task_query: Arc<B>) -> Self {
        Self {
            task_command,
            task_query,
        }
    }
}

impl<A, B> TaskCommand for TaskUseCase<A, B>
where
    A: task_command::TaskCommand,
    B: task_query::TaskQuery,
{
    async fn create(&self, content: String) -> Result<TaskId, AppError> {
        let x = self.task_command.create(&content).await?;

        Ok(x)
    }

    async fn update(&self, task: Task) -> Result<(), AppError> {
        self.task_command.update(&task).await?;

        Ok(())
    }
}

impl<A, B> TaskQuery for TaskUseCase<A, B>
where
    A: task_command::TaskCommand,
    B: task_query::TaskQuery,
{
    async fn get(&self, task_id: TaskId) -> Result<task_query::Task, AppError> {
        let x = self.task_query.get(&task_id).await?;

        Ok(x)
    }

    async fn get_list(&self) -> Result<Vec<task_query::Task>, AppError> {
        let x = self.task_query.get_list().await?;

        Ok(x)
    }
}
