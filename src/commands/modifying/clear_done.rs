use super::super::{config, entities, storage::ModifyStorage, Command};
use super::BaseModifyCommand;
pub struct ClearDoneCommand {
    base: BaseModifyCommand,
}
impl Command for ClearDoneCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let last_id = self.base.storage.add_task("empty")? - 1;
        dbg!();
        self.base.storage.delete_task(&(last_id + 1))?;
        for id in 1..=last_id {
            let now_task = self.base.storage.read_task(&id);
            let now_task = match now_task {
                Ok(task) => task,
                Err(_) => continue,
            };

            if let entities::task::TaskStatus::Done = now_task.get_status() {
                self.base.storage.delete_task(&id)?
            }
        }
        Ok(())
    }
}
pub fn new(config: config::Config, storage: Box<dyn ModifyStorage>) -> ClearDoneCommand {
    ClearDoneCommand {
        base: BaseModifyCommand::new(config, storage),
    }
}
