use super::super::{config, entities, storage::ModifyStorage, Command};
use super::BaseModifyCommand;
pub struct MarkDoneCommand {
    base: BaseModifyCommand,
}
impl Command for MarkDoneCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let task_id = match self.base.config.task_id {
            Some(id) => id,
            None => return Err("Task id is required".into()),
        };
        let mut task = self.base.storage.read_task(&task_id)?;
        task.set_status(entities::task::TaskStatus::Done);
        self.base.storage.update_task(&task_id, task)?;
        Ok(())
    }
}
pub fn new(config: config::Config, storage: Box<dyn ModifyStorage>) -> MarkDoneCommand {
    MarkDoneCommand {
        base: BaseModifyCommand::new(config, storage),
    }
}
