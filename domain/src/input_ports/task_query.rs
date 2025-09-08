use crate::app_errors::AppError;
use crate::models::task::TaskId;
use crate::models::task_query::Task;

pub trait TaskQuery: Send + Sync + Clone + 'static {
    fn get(&self, task_id: TaskId) -> impl Future<Output = Result<Task, AppError>> + Send;

    fn get_list(&self) -> impl Future<Output = Result<Vec<Task>, AppError>> + Send;
}
