use crate::{config, entities, storage};

use super::super::{BaseModifyCommand, Command};

pub struct MarkInProgressCommand {
    base: BaseModifyCommand,
}
impl Command for MarkInProgressCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let task_id = match self.base.config.task_id {
            Some(id) => id,
            None => return Err("Task id is required".into()),
        };

        let mut task = self.base.storage.read_task(&task_id)?;

        task.set_status(entities::task::TaskStatus::InProgress);

        self.base.storage.update_task(&task_id, task)?;
        Ok(())
    }
}
pub fn new(
    config: config::Config,
    storage: Box<dyn storage::ModifyStorage>,
) -> MarkInProgressCommand {
    MarkInProgressCommand {
        base: BaseModifyCommand::new(config, storage),
    }
}
