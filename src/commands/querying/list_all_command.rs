use std::error::Error;

use crate::{config, storage::QueryStorage};

use super::super::{BaseQueryCommand, Command};

pub struct ListAllCommand {
    base: BaseQueryCommand,
}

impl Command for ListAllCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let last_task_number = self.base.storage.last_id()?;
        println!("Tasks -----------------");

        for id in 1..=last_task_number {
            let task = self.base.storage.read_task(&id);
            match task {
                Ok(task) => self.base.print_task(task, id),
                Err(_) => continue,
            }
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
