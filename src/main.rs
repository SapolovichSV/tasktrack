//use config::Config;
use std::env;

mod commands;
mod config;
mod entities;
mod storage;

fn main() {
    let config = config::Config::build(env::args()).unwrap_or_else(|x| {
        eprintln!("Error: {}", x);
        std::process::exit(1)
    });
    let stg = match storage::fsstorage::new("./tasks".to_string()) {
        Ok(storage) => Box::new(storage),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    let stg_types = match config.command.kind {
        entities::command_kind::CommandKind::Modifying => storage::StorageType::Modify(stg),
        entities::command_kind::CommandKind::Querying => storage::StorageType::Query(stg),
    };
    let cmd = commands::commands_factory::create_command(config, stg_types).unwrap_or_else(|x| {
        eprintln!("Error: {}", x);
        std::process::exit(1)
    });
    cmd.execute().expect("good");
}
