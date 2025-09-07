use crate::app_errors::AppError;
use crate::input_ports::task_command::TaskCommand;
use crate::models::task::TaskId;
use crate::models::task_command::Task;
use crate::output_ports::task_command;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TaskUseCase<A: task_command::TaskCommand> {
    task_command: Arc<A>,
}

impl<A> TaskUseCase<A>
where
    A: task_command::TaskCommand,
{
    pub fn new(task_command: Arc<A>) -> Self {
        Self {
            task_command,
        }
    }
}

impl<A> TaskCommand for TaskUseCase<A>
where
    A: task_command::TaskCommand,
{
    async fn create(&self, text: String) -> Result<TaskId, AppError> {
        let x = self.task_command.create(&text).await?;

        Ok(x)
    }

    async fn update(&self, task: Task) -> Result<(), AppError> {
        self.task_command.update(&task).await?;

        Ok(())
    }
}
