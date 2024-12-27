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
    fn add_task(&self, task_name: &str) -> Result<u8, Box<dyn Error>> {
        let last_id = match self.find_last_id() {
            Ok(id) => id,
            Err(_) => 0,
        };
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
        match self.read_task(task_id) {
            Ok(task) => Ok(task),
            Err(e) => Err(e),
        }
    }
}
impl QueryStorage for Storage {
    fn read_task(&self, task_id: &u8) -> Result<Task, Box<dyn Error>> {
        self.read_task(task_id)
    }
    fn last_id(&self) -> Result<u8, Box<dyn Error>> {
        self.find_last_id()
    }
}
impl Storage {
    fn find_last_id(&self) -> Result<u8, Box<dyn Error>> {
        let last_id = fs::read_dir(&self.root)?.into_iter().last();
        let last_id = match last_id {
            Some(entry) => {
                let entry = entry?;

                let file_name = entry.file_name();
                let file_name = match file_name.to_str() {
                    Some(name) => name,
                    None => return Err("Invalid file name".into()),
                };
                let id: u8 = file_name
                    .strip_prefix("task_")
                    .ok_or("Invalid file name")?
                    .strip_suffix(".json")
                    .ok_or("Invalid file name")?
                    .parse()?;
                id
            }
            None => return Err("Can't parse dir for last id")?,
        };
        Ok(last_id)
    }

    fn form_new_task_path(&self, last_id: u8) -> (PathBuf, u8) {
        let new_id = last_id + 1;
        let new_part_path = format!("task_{}.json", new_id);
        (self.root.join(new_part_path), new_id)
    }

    fn form_new_task(&self, last_id: u8, task_name: &str) -> task::Task {
        let mut task = task::new(task_name.to_owned());
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
