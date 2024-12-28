pub mod add_command;
pub mod clear_done;
pub mod delete_command;
pub mod mark_done_command;
pub mod mark_in_progess_command;
pub mod update_command;
use super::{config, storage};
pub struct BaseModifyCommand {
    config: config::Config,
    storage: Box<dyn storage::ModifyStorage>,
}
impl BaseModifyCommand {
    pub fn new(
        config: config::Config,
        storage: Box<dyn storage::ModifyStorage>,
    ) -> BaseModifyCommand {
        BaseModifyCommand { config, storage }
    }
}
