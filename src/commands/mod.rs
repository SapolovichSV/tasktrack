use std::error::Error;

pub mod commands_factory;

mod modifying;
mod querying;

use crate::config;
use crate::entities;
use crate::storage;
pub trait Command {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
}
