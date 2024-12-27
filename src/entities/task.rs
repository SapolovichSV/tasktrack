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
    pub fn set_name(&mut self, name: String) {
        self.task_name = name;
    }
    pub fn get_status(&self) -> &TaskStatus {
        &self.task_status
    }
    pub fn get_name(&self) -> &String {
        &self.task_name
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Done,
    InProgress,
    NotDone,
}
impl Clone for TaskStatus {
    fn clone(&self) -> TaskStatus {
        match self {
            TaskStatus::Done => TaskStatus::Done,
            TaskStatus::InProgress => TaskStatus::InProgress,
            TaskStatus::NotDone => TaskStatus::NotDone,
        }
    }
}
impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TaskStatus::Done => write!(f, "Done"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::NotDone => write!(f, "Not Done"),
        }
    }
}
pub fn new(task_name: String) -> Task {
    Task {
        task_name,
        task_id: 0,
        task_status: TaskStatus::NotDone,
    }
}
