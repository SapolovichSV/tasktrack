use std::error::Error;

use crate::{config::Config, entities::commands::Commands};

use super::{storage, Model};
const ROOTPATH: &str = "./tasks";
pub struct ModifyModel {
    command: Commands,
    task_id: u8,
    task_name: String,
}
impl Model for ModifyModel {
    fn execute(&self) {
        println!(
            "Modifying with command {:?} task {:?} with id {:?}",
            self.command, self.task_name, self.task_id
        );
        match self.command {
            Commands::Add => match storage::fsstorage::new(String::from(ROOTPATH)) {
                Ok(storage) => match self.add_task(&storage) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error: {}", e),
                },
                Err(e) => eprintln!("Error: {}", e),
            },
            _ => (),
        }
    }
}
impl ModifyModel {
    fn add_task<'a, T: storage::ModifyStorage>(
        &'a self,
        storag: &'a T,
    ) -> Result<(), Box<dyn Error + 'a>> {
        match storag.add_task(&self.task_name) {
            Ok(task_id) => {
                println!("Task added with id {}", task_id);
                Ok(())
            }
            Err(e) => return Err(e),
        }
    }
}
pub fn new(config: Config) -> ModifyModel {
    ModifyModel {
        command: config.command.name,
        task_id: config.task_id.unwrap(),
        task_name: config.task_name.unwrap(),
    }
}
