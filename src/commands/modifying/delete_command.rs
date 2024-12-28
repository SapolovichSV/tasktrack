use super::super::{config, storage::ModifyStorage, Command};
use super::BaseModifyCommand;
pub struct DeleteCommand {
    base: BaseModifyCommand,
}
impl Command for DeleteCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let delete_id = match self.base.config.task_id {
            Some(id) => id,
            None => return Err("Task id is required".into()),
        };
        self.base.storage.delete_task(&delete_id)?;
        Ok(())
    }
}
pub fn new(config: config::Config, storage: Box<dyn ModifyStorage>) -> DeleteCommand {
    DeleteCommand {
        base: BaseModifyCommand::new(config, storage),
    }
}
