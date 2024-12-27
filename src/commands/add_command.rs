use std::error::Error;

use crate::{config::config::Config, storage::ModifyStorage};

use super::Command;

pub struct AddCommand {
    config: Config,
    storage: Box<dyn ModifyStorage>,
}

impl Command for AddCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let task_name = match &self.config.task_name {
            Some(name) => name,
            None => {
                return Err("Task name is required".into());
            }
        };
        let task_id = self.storage.add_task(&task_name)?;
        println!("Task added with id: {}", task_id);
        Ok(())
    }
}
pub fn new(config: Config, storage: Box<dyn ModifyStorage>) -> AddCommand {
    AddCommand { config, storage }
}
