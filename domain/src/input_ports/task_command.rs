use crate::app_errors::AppError;
use crate::models::task::TaskId;
use crate::models::task_command;

pub trait TaskCommand: Send + Sync + Clone + 'static {
    fn create(&self, content: String) -> impl Future<Output = Result<TaskId, AppError>> + Send;

    fn update(&self, task: task_command::Task)
    -> impl Future<Output = Result<(), AppError>> + Send;
}
