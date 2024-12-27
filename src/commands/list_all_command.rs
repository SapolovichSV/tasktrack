use std::error::Error;

use crate::{config, storage::QueryStorage};

use super::{BaseQueryCommand, Command};

pub struct ListAllCommand {
    base: BaseQueryCommand,
}

impl Command for ListAllCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let tasks_count = self.base.storage.len_storage()?;
        println!("Total tasks: {}", tasks_count);

        for i in 1..=tasks_count {
            let task = self.base.storage.read_task(&i.try_into()?)?;
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
    ListAllCommand {
        base: BaseQueryCommand::new(storage),
    }
}
