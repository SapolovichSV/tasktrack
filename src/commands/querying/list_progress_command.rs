use super::super::{entities, BaseQueryCommand, Command};
pub struct ListProgressCommand {
    base: BaseQueryCommand,
}
impl Command for ListProgressCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let last_task_id = self.base.storage.last_id()?;
        for id in 1..=last_task_id {
            if self
                .base
                .task_status_is(id, entities::task::TaskStatus::InProgress)?
            {
                let task = self.base.storage.read_task(&id)?;
                self.base.print_task(task, id);
            }
        }
        Ok(())
    }
}
