use std::error::Error;

use super::entities::Task;
pub mod fsstorage;
pub trait ModifyStorage {
    fn add_task(&self, task_name: &String) -> Result<u8, Box<dyn Error>>;
    fn update_task(&self, task_id: &u8) -> Result<(), Box<dyn Error>>;
    fn delete_task(&self, task_id: &u8) -> Result<(), Box<dyn Error>>;
    fn mark_in_progess(&self, task_id: &u8) -> Result<(), Box<dyn Error>>;
    fn mark_done(&self, task_id: &u8) -> Result<(), Box<dyn Error>>;
    fn clear_done(&self) -> Result<(), Box<dyn Error>>;
}
pub trait QueryStorage {
    fn get_all_tasks() -> Result<Vec<Task>, Box<dyn Error>>;
    fn get_done_tasks() -> Result<Vec<Task>, Box<dyn Error>>;
    fn get_not_done_tasks() -> Result<Vec<Task>, Box<dyn Error>>;
    fn get_progress_tasks() -> Result<Vec<Task>, Box<dyn Error>>;
}
