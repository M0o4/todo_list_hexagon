use crate::app_errors::OutputPortError;
use crate::models::task::TaskId;
pub(crate) use crate::models::task_query::Task;

pub trait TaskQuery: Send + Sync + Clone + 'static {
    fn get(&self, task_id: &TaskId) -> impl Future<Output = Result<Task, OutputPortError>> + Send;

    fn get_list(&self) -> impl Future<Output = Result<Vec<Task>, OutputPortError>> + Send;
}
