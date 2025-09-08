use domain::app_errors::OutputPortError;
use domain::models::task::TaskId;
use domain::models::task_query::Task;
use domain::models::utils::Datetime;
use domain::output_ports::task_query::TaskQuery;
use sqlx::types::time::OffsetDateTime;

#[derive(Clone, Debug)]
pub struct TaskQueryMock {}

impl TaskQuery for TaskQueryMock {
    async fn get(&self, task_id: &TaskId) -> Result<Task, OutputPortError> {
        let task = Task::new(
            TaskId(ulid::Ulid::new()),
            "".to_string(),
            false,
            Datetime(OffsetDateTime::now_utc()),
        );

        Ok(task)
    }

    async fn get_list(&self) -> Result<Vec<Task>, OutputPortError> {
        Ok(vec![])
    }
}
