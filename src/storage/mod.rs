use std::error::Error;

use crate::entities::task::Task;
pub mod fsstorage;
pub enum StorageType {
    Modify(Box<dyn ModifyStorage>),
    Query(Box<dyn QueryStorage>),
}
pub trait ModifyStorage {
    fn add_task(&self, task_name: &str) -> Result<u8, Box<dyn Error>>;
    fn read_task(&self, task_id: &u8) -> Result<Task, Box<dyn Error>>;
    fn update_task(&self, task_id: &u8, updated_task: Task) -> Result<(), Box<dyn Error>>;
    fn delete_task(&self, task_id: &u8) -> Result<(), Box<dyn Error>>;
}
pub trait QueryStorage {
    fn read_task(&self, task_id: &u8) -> Result<Task, Box<dyn Error>>;
    fn len_storage(&self) -> Result<usize, Box<dyn Error>>;
}
