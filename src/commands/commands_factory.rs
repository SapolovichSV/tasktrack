use std::error::Error;

use super::modifying::{
    add_command, clear_done, delete_command, mark_done_command, mark_in_progess_command,
    update_command,
};
use super::querying::{list_all_command, list_done_command, list_not_done_command};
use super::Command;
use super::{config, entities, storage::StorageType};

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
        entities::commands::Commands::Delete => Ok(Box::new(delete_command::new(config, storage))),

        entities::commands::Commands::MarkInProgress => {
            Ok(Box::new(mark_in_progess_command::new(config, storage)))
        }
        entities::commands::Commands::MarkDone => {
            Ok(Box::new(mark_done_command::new(config, storage)))
        }
        entities::commands::Commands::ClearDone => Ok(Box::new(clear_done::new(config, storage))),
        _ => Err("Invalid command".into()),
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
            Ok(Box::new(list_done_command::new(config, storage)))
        }
        entities::commands::Commands::ListNotDone => {
            Ok(Box::new(list_not_done_command::new(config, storage)))
        }
        entities::commands::Commands::ListProgress => {
            todo!()
        }
        _ => Err("Invalid command".into()),
    }
}
