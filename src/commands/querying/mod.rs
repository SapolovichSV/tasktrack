pub mod list_all_command;
pub mod list_done_command;
pub mod list_not_done_command;
pub mod list_progress_command;
use super::{entities, storage};
use std::error::Error;
pub struct BaseQueryCommand {
    storage: Box<dyn storage::QueryStorage>,
}
impl BaseQueryCommand {
    pub fn new(storage: Box<dyn storage::QueryStorage>) -> BaseQueryCommand {
        BaseQueryCommand { storage }
    }
    fn task_status_is(
        &self,
        task_id: u8,
        task_status: entities::task::TaskStatus,
    ) -> Result<bool, Box<dyn Error>> {
        let task = self.storage.read_task(&task_id);
        let task = match task {
            Ok(task) => task,
            Err(_) => return Ok(false),
        };
        // match task_status {
        //
        // }
        let current_status = task.get_status();
        Ok(*current_status == task_status)
    }
    fn print_task(&self, task: entities::task::Task, task_id: u8) {
        println!(
            "Task id: {} name: {} status: {}",
            task_id,
            task.get_name(),
            task.get_status()
        );
    }
}
