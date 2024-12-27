use crate::{config, entities, storage};

use super::{BaseModifyCommand, Command};

pub struct UpdateNameCommand {
    base: BaseModifyCommand,
}

impl Command for UpdateNameCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let task_id = match self.base.config.task_id {
            Some(id) => id,
            None => {
                return Err("Task id is required".into());
            }
        };

        let old_task = self.base.storage.read_task(&task_id)?;
        let new_task_name = match &self.base.config.task_name {
            Some(name) => name,
            None => return Err("Task name is required".into()),
        };

        let mut updated_task = entities::task::new(new_task_name.clone());
        updated_task.set_status(old_task.get_status().clone());
        println!(
            "Task updated with id: {} new name is {}",
            task_id, new_task_name
        );
        self.base.storage.update_task(&task_id, updated_task)
    }
}
pub fn new(config: config::Config, storage: Box<dyn storage::ModifyStorage>) -> UpdateNameCommand {
    UpdateNameCommand {
        base: BaseModifyCommand::new(config, storage),
    }
}
