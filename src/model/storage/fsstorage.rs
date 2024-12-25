use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use crate::model::entities::{self};

use super::ModifyStorage;

pub fn new(root: String) -> Result<Storage, Box<dyn Error>> {
    let path = PathBuf::from(root);
    match fs::exists(&path) {
        Ok(true) => Ok(Storage { root: path }),
        Ok(false) => {
            fs::create_dir_all(&path)?;
            Ok(Storage { root: path })
        }
        Err(e) => Err(Box::new(e)),
    }
}
pub struct Storage {
    root: PathBuf,
}
impl ModifyStorage for Storage {
    fn add_task(&self, task_name: &String) -> Result<u8, Box<dyn Error>> {
        match self.find_last_id() {
            Ok(last_id) => {
                let task = self.form_new_task(last_id, task_name);
                let (file_path, new_id) = self.form_new_task_path(&last_id);

                let mut file = File::create(file_path)?;
                let json_data = serde_json::to_string(&task)?;

                file.write(json_data.as_bytes())?;
                Ok(new_id)
            }
            Err(e) => Err(e),
        }
    }

    fn update_task(&self, task_id: &u8) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn clear_done(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn delete_task(&self, task_id: &u8) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn mark_done(&self, task_id: &u8) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn mark_in_progess(&self, task_id: &u8) -> Result<(), Box<dyn Error>> {
        todo!()
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

    fn form_new_task_path(&self, last_id: &u8) -> (PathBuf, u8) {
        let new_id = last_id + 1;
        let new_part_path = "task_".to_string() + &new_id.to_string() + ".json";
        (self.root.join(new_part_path), last_id + 1)
    }

    fn form_new_task(&self, last_id: u8, task_name: &String) -> entities::Task {
        let new_id = last_id + 1;
        let mut task = entities::new(task_name.clone());
        task.set_id(new_id.clone());
        task.set_status(entities::TaskStatus::NotDone);
        task
    }
}
