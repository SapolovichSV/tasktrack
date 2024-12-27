use std::error::Error;
mod add_command;
mod clear_done;
pub mod commands_factory;
mod delete_command;
mod list_all_command;
mod list_done_command;
mod mark_done_command;
mod mark_in_progess_command;
mod update_command;
use crate::config;
use crate::entities;
use crate::storage;
pub trait Command {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
}
pub struct BaseModifyCommand {
    config: config::Config,
    storage: Box<dyn storage::ModifyStorage>,
}
impl BaseModifyCommand {
    pub fn new(
        config: config::Config,
        storage: Box<dyn storage::ModifyStorage>,
    ) -> BaseModifyCommand {
        BaseModifyCommand { config, storage }
    }
}
pub struct BaseQueryCommand {
    storage: Box<dyn storage::QueryStorage>,
}
impl BaseQueryCommand {
    pub fn new(storage: Box<dyn storage::QueryStorage>) -> BaseQueryCommand {
        BaseQueryCommand { storage }
    }
    fn task_status_is_done(&self, task_id: u8) -> Result<bool, Box<dyn Error>> {
        let task = self.storage.read_task(&task_id);
        let task = match task {
            Ok(task) => task,
            Err(_) => return Ok(false),
        };
        if let entities::task::TaskStatus::Done = task.get_status() {
            return Ok(true);
        } else {
            return Ok(false);
        }
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
