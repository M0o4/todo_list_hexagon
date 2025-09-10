use anyhow::anyhow;
use domain::app_errors::OutputPortError;
use domain::models::task::TaskId;
use domain::models::task_command::Task;
use domain::output_ports::task_command::TaskCommand;
use sqlx::{Pool, Postgres};

#[derive(Clone, Debug)]
pub struct SqlxTaskCommand {
    pool: Pool<Postgres>,
}

impl SqlxTaskCommand {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

impl TaskCommand for SqlxTaskCommand {
    async fn create(&self, content: &str) -> Result<TaskId, OutputPortError> {
        let task_id = ulid::Ulid::new();
        let id_bytes = task_id.to_bytes();

        let x = sqlx::query!(
            r#"INSERT INTO tasks(task_id, content) VALUES ($1, $2) RETURNING task_id"#,
            &id_bytes[..],
            content
        )
        .fetch_one(&self.pool)
        .await?;

        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&x.task_id);
        let ulid = ulid::Ulid::from_bytes(bytes);

        Ok(TaskId(ulid))
    }

    async fn update(&self, task: &Task) -> Result<(), OutputPortError> {
        let id_bytes = task.id_bytes();

        let result = sqlx::query!(
            r#"UPDATE tasks SET content = $2, completed = $3 WHERE task_id = $1"#,
            &id_bytes[..],
            task.content(),
            task.completed()
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(OutputPortError::NotFound(anyhow!("task not found")));
        }

        Ok(())
    }

    async fn delete(&self, task_id: &TaskId) -> Result<(), OutputPortError> {
        let id_bytes = task_id.0.to_bytes();

        sqlx::query!(r#"DELETE FROM tasks WHERE task_id = $1"#, &id_bytes[..])
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
