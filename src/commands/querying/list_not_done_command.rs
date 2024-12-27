use super::super::{config, entities, storage, BaseQueryCommand, Command};

pub struct ListNotDoneCommand {
    base: BaseQueryCommand,
}
impl Command for ListNotDoneCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let last_task_id = self.base.storage.last_id()?;
        for id in 1..=last_task_id {
            if self
                .base
                .task_status_is(id, entities::task::TaskStatus::NotDone)?
            {
                let task = self.base.storage.read_task(&id)?;
                self.base.print_task(task, id);
            }
        }
        Ok(())
    }
}
pub fn new(config: config::Config, storage: Box<dyn storage::QueryStorage>) -> ListNotDoneCommand {
    let _config = config;
    ListNotDoneCommand {
        base: BaseQueryCommand::new(storage),
    }
}
