use crate::app_errors::OutputPortError;
use crate::models::task::TaskId;
use crate::models::task_command;

pub trait TaskCommand: Send + Sync + Clone + 'static {
    fn create(&self, content: &str)
    -> impl Future<Output = Result<TaskId, OutputPortError>> + Send;

    fn update(
        &self,
        task: &task_command::Task,
    ) -> impl Future<Output = Result<(), OutputPortError>> + Send;
}
