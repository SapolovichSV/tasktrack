use std::error::Error;

use crate::{config, storage::QueryStorage};

use super::Command;

pub struct ListAllCommand {
    storage: Box<dyn QueryStorage>,
}

impl Command for ListAllCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let tasks_count = self.storage.len_storage()?;
        println!("Total tasks: {}", tasks_count);

        for i in 1..=tasks_count {
            let task = self.storage.read_task(&i.try_into()?)?;
            println!(
                "Task id: {} name: {} status: {}",
                i,
                task.get_name(),
                task.get_status()
            );
        }
        Ok(())
    }
}
pub fn new(config: config::Config, storage: Box<dyn QueryStorage>) -> ListAllCommand {
    let _config = config;
    ListAllCommand { storage }
}
