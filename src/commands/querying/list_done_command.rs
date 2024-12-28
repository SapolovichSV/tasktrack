use super::super::{config, entities, storage::QueryStorage, Command};
use super::BaseQueryCommand;
use std::error::Error;
pub struct ListDoneCommand {
    base: BaseQueryCommand,
}
impl Command for ListDoneCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let last_task_number = self.base.storage.last_id()?;
        println!("Tasks -----------------");
        for id in 1..last_task_number {
            match self
                .base
                .task_status_is(id, entities::task::TaskStatus::Done)
            {
                Ok(true) => {
                    let task = self.base.storage.read_task(&id)?;
                    self.base.print_task(task, id);
                }
                Ok(false) => continue,
                Err(_) => continue,
            }
        }
        Ok(())
    }
}
pub fn new(config: config::Config, storage: Box<dyn QueryStorage>) -> ListDoneCommand {
    let _config = config;
    ListDoneCommand {
        base: BaseQueryCommand::new(storage),
    }
}
