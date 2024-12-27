use std::error::Error;

use crate::{config::config, entities, storage::StorageType};

use super::add_command;
use super::list_all_command;
use super::update_command;
use super::Command;

pub fn create_command(
    config: config::Config,
    storage: StorageType,
) -> Result<Box<dyn Command>, Box<dyn Error>> {
    match config.command.kind {
        entities::command_kind::CommandKind::Modifying => create_modifying_command(config, storage),
        entities::command_kind::CommandKind::Querying => create_querying_command(config, storage),
    }
}
fn create_modifying_command(
    config: config::Config,
    storage: StorageType,
) -> Result<Box<dyn Command>, Box<dyn Error>> {
    let storage = match storage {
        StorageType::Modify(storage) => storage,
        _ => return Err("Invalid storage type".into()),
    };

    match config.command.name {
        entities::commands::Commands::Add => Ok(Box::new(add_command::new(config, storage))),
        entities::commands::Commands::Update => Ok(Box::new(update_command::new(config, storage))),
        entities::commands::Commands::Delete => {
            todo!()
        }
        entities::commands::Commands::MarkInProgress => {
            todo!()
        }
        entities::commands::Commands::MarkDone => {
            todo!()
        }
        entities::commands::Commands::ClearDone => {
            todo!()
        }
        _ => return Err("Invalid command".into()),
    }
}

fn create_querying_command(
    config: config::Config,
    storage: StorageType,
) -> Result<Box<dyn Command>, Box<dyn Error>> {
    let storage = match storage {
        StorageType::Query(storage) => storage,
        _ => return Err("Invalid storage type".into()),
    };
    match config.command.name {
        entities::commands::Commands::ListAll => {
            Ok(Box::new(list_all_command::new(config, storage)))
        }
        entities::commands::Commands::ListDone => {
            todo!()
        }
        entities::commands::Commands::ListNotDone => {
            todo!()
        }
        entities::commands::Commands::ListProgress => {
            todo!()
        }
        _ => Err("Invalid command".into()),
    }
}
