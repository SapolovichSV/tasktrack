use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use super::{ModifyStorage, QueryStorage};
use crate::entities::task::{self, Task};

pub fn new(root: String) -> Result<Storage, Box<dyn Error>> {
    let path = PathBuf::from(root);
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(Storage { root: path })
}
#[derive(Debug)]
pub struct Storage {
    root: PathBuf,
}
impl ModifyStorage for Storage {
    fn add_task(&self, task_name: &String) -> Result<u8, Box<dyn Error>> {
        let last_id = self.find_last_id()?;
        let task = self.form_new_task(last_id, task_name);
        let (file_path, new_id) = self.form_new_task_path(last_id);

        let mut file = File::create(file_path)?;
        let json_data = serde_json::to_string(&task)?;

        file.write_all(json_data.as_bytes())?;
        Ok(new_id)
    }

    fn update_task(&self, task_id: &u8, updated_task: Task) -> Result<(), Box<dyn Error>> {
        let file_name = format!("task_{}.json", task_id);
        let file_path = self.root.join(file_name);

        let mut task = self.read_task(task_id)?;
        task.set_name(updated_task.get_name().clone());
        task.set_status(updated_task.get_status().clone());

        let mut file = File::create(file_path)?;
        let json_data = serde_json::to_string(&task)?;
        file.write_all(json_data.as_bytes())?;

        Ok(())
    }
    fn delete_task(&self, task_id: &u8) -> Result<(), Box<dyn Error>> {
        let file_name = format!("task_{}.json", task_id);
        let file_path = self.root.join(file_name);
        fs::remove_file(file_path)?;
        Ok(())
    }
    fn read_task(&self, task_id: &u8) -> Result<Task, Box<dyn Error>> {
        self.read_task(task_id)
    }
}
impl QueryStorage for Storage {
    fn read_task(&self, task_id: &u8) -> Result<Task, Box<dyn Error>> {
        self.read_task(task_id)
    }
    fn len_storage(&self) -> Result<usize, Box<dyn Error>> {
        let len = fs::read_dir(&self.root)?.count();
        Ok(len)
    }
}
impl Storage {
    fn find_last_id(&self) -> Result<u8, Box<dyn Error>> {
        let last_id = fs::read_dir(&self.root)?
            .count()
            .try_into()
            .map_err(|_| "Error: Cannot convert to u8")?;
        println!("Last id: {}", last_id);
        Ok(last_id)
    }

    fn form_new_task_path(&self, last_id: u8) -> (PathBuf, u8) {
        let new_id = last_id + 1;
        let new_part_path = format!("task_{}.json", new_id);
        (self.root.join(new_part_path), new_id)
    }

    fn form_new_task(&self, last_id: u8, task_name: &String) -> task::Task {
        let mut task = task::new(task_name.clone());
        task.set_id(last_id + 1);
        task.set_status(task::TaskStatus::NotDone);
        task
    }
    fn read_task(&self, task_id: &u8) -> Result<Task, Box<dyn Error>> {
        let file_name = format!("task_{}.json", task_id);
        let file_path = self.root.join(file_name);
        let file = File::open(file_path)?;
        let task: Task = serde_json::from_reader(file)?;
        Ok(task)
    }
}
