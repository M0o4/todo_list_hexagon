use domain::app_errors::OutputPortError;
use domain::models::task::TaskId;
use domain::models::task_command::Task;
use domain::output_ports::task_command::TaskCommand;

#[derive(Clone, Debug)]
pub struct TaskCommandMock {}

impl TaskCommand for TaskCommandMock {
    async fn create(&self, content: &str) -> Result<TaskId, OutputPortError> {
        Ok(TaskId(ulid::Ulid::new()))
    }

    async fn update(&self, task: &Task) -> Result<(), OutputPortError> {
        Ok(())
    }

    async fn delete(&self, task_id: &TaskId) -> Result<(), OutputPortError> {
        Ok(())
    }
}
