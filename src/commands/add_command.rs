use std::error::Error;

use crate::{config, storage::ModifyStorage};

use super::{BaseModifyCommand, Command};

pub struct AddCommand {
    base: BaseModifyCommand,
}

impl Command for AddCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let task_name = match &self.base.config.task_name {
            Some(name) => name,
            None => {
                return Err("Task name is required".into());
            }
        };
        let task_id = self.base.storage.add_task(task_name)?;
        println!("Task added with id: {}", task_id);
        Ok(())
    }
}
pub fn new(config: config::Config, storage: Box<dyn ModifyStorage>) -> AddCommand {
    AddCommand {
        base: BaseModifyCommand::new(config, storage),
    }
}
