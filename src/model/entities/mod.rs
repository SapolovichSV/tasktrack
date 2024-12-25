use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    task_name: String,
    task_id: u8,
    task_status: TaskStatus,
}
impl Task {
    pub fn set_id(&mut self, id: u8) {
        self.task_id = id;
    }
    pub fn set_status(&mut self, status: TaskStatus) {
        self.task_status = status;
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Done,
    InProgress,
    NotDone,
}
pub fn new(task_name: String) -> Task {
    Task {
        task_name,
        task_id: 0,
        task_status: TaskStatus::NotDone,
    }
}
