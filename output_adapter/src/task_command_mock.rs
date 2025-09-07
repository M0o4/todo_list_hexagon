use domain::app_errors::OutputPortError;
use domain::models::task::TaskId;
use domain::models::task_command::Task;
use domain::output_ports::task_command::TaskCommand;

#[derive(Clone, Debug)]
pub struct TaskCommandMock {}

impl TaskCommand for TaskCommandMock {
    async fn create(&self, text: &str) -> Result<TaskId, OutputPortError> {
        Ok(TaskId(ulid::Ulid::new()))
    }

    async fn update(&self, task: &Task) -> Result<(), OutputPortError> {
        Ok(())
    }
}