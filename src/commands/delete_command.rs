use crate::config::config;
use crate::storage::ModifyStorage;

use super::Command;
pub struct DeleteCommand {
    config: config::Config,
    storage: Box<dyn ModifyStorage>,
}
impl Command for DeleteCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let delete_id = match self.config.task_id {
            Some(id) => id,
            None => return Err("Task id is required".into()),
        };
        self.storage.delete_task(&delete_id)?;
        Ok(())
    }
}
