use std::error::Error;

use super::{storage, Config, Model};
use crate::entities::commands::Commands;
const ROOTPATH: &str = "./tasks";
pub struct ModifyModel {
    command: Commands,
    task_id: Option<u8>,
    task_name: Option<String>,
}
impl Model for ModifyModel {
    fn execute(&self) {
        println!(
            "Modifying with command {:?} task {:?} with id {:?}",
            self.command, self.task_name, self.task_id
        );
        if let Commands::Add = self.command {
            if let Some(task_name) = &self.task_name {
                match storage::fsstorage::new(String::from(ROOTPATH)) {
                    Ok(storage) => match self.add_task(&storage, task_name) {
                        Ok(_) => (),
                        Err(e) => eprintln!("Error: {}", e),
                    },
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
    }
}
impl ModifyModel {
    fn add_task<T: storage::ModifyStorage>(
        &self,
        storage: &T,
        task_name: &String,
    ) -> Result<(), Box<dyn Error>> {
        match storage.add_task(task_name) {
            Ok(task_id) => {
                println!("Task added with id {}", task_id);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
pub fn new(config: Config) -> ModifyModel {
    ModifyModel {
        command: config.command.name,
        task_id: config.task_id,
        task_name: config.task_name,
    }
}
