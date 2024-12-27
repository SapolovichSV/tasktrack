use std::error::Error;
mod add_command;
pub mod commands_factory;
mod delete_command;
mod list_all_command;
mod update_command;
pub trait Command {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
}
