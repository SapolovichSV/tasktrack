use std::error::Error;
mod add_command;
pub mod commands_factory;
mod delete_command;
mod list_all_command;
mod mark_in_progess_command;
mod update_command;
use crate::config;
use crate::storage;
pub trait Command {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
}
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
pub struct BaseQueryCommand {
    storage: Box<dyn storage::QueryStorage>,
}
impl BaseQueryCommand {
    pub fn new(storage: Box<dyn storage::QueryStorage>) -> BaseQueryCommand {
        BaseQueryCommand { storage }
    }
}
