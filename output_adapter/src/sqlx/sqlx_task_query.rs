use domain::app_errors::OutputPortError;
use domain::models::task::TaskId;
use domain::models::task_query::Task;
use domain::models::utils::Datetime;
use domain::output_ports::task_query::TaskQuery;
use sqlx::{Pool, Postgres};
use ulid::Ulid;

#[derive(Clone, Debug)]
pub struct SqlxTaskQuery {
    pool: Pool<Postgres>,
}

impl SqlxTaskQuery {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

impl TaskQuery for SqlxTaskQuery {
    async fn get(&self, task_id: &TaskId) -> Result<Task, OutputPortError> {
        let id = task_id.0.to_bytes();

        let x = sqlx::query!(
            "SELECT task_id, content, completed, created_at FROM tasks WHERE task_id = $1",
            &id[..]
        )
        .fetch_one(&self.pool)
        .await?;

        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&x.task_id);

        Ok(Task::new(
            TaskId(Ulid::from_bytes(bytes)),
            x.content,
            x.completed,
            Datetime(x.created_at),
        ))
    }

    async fn get_list(&self) -> Result<Vec<Task>, OutputPortError> {
        let x = sqlx::query!(r#"SELECT task_id, content, completed, created_at FROM tasks"#)
            .fetch_all(&self.pool)
            .await?;

        let tasks = x
            .into_iter()
            .map(|task| {
                let mut bytes = [0u8; 16];
                bytes.copy_from_slice(&task.task_id);

                Task::new(
                    TaskId(Ulid::from_bytes(bytes)),
                    task.content,
                    task.completed,
                    Datetime(task.created_at),
                )
            })
            .collect();

        Ok(tasks)
    }
}
