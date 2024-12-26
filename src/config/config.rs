use crate::entities::{command_kind::CommandKind, commands::Commands};
use std::env;
#[derive(Debug)]

pub struct Config {
    pub command: Command,
    pub task_id: Option<u8>,
    pub task_name: Option<String>,
}
#[derive(Debug)]
pub struct Command {
    pub kind: CommandKind,
    pub name: Commands,
}
impl Config {
    pub fn build(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let command = match args.next() {
            Some(command) => match Command::parse_command(command) {
                Ok(command) => command,
                Err(e) => return Err(e),
            },
            None => return Err("No command provided"),
        };
        match command.kind {
            CommandKind::Querying => Ok(Config {
                command,
                task_id: None,
                task_name: None,
            }),
            CommandKind::Modifying => {
                let task_id: u8 = Self::parse_task_id(&mut args)?;
                let task_name = Self::parse_task_name(&mut args)?;
                Ok(Config {
                    command: command,
                    task_id: Some(task_id),
                    task_name: Some(task_name),
                })
            }
        }
    }
    fn parse_task_id(args: &mut impl Iterator<Item = String>) -> Result<u8, &'static str> {
        match args.next() {
            Some(task_id) => match task_id.parse() {
                Ok(task_id) => Ok(task_id),
                Err(_) => Err("Error while try to parse task id"),
            },
            None => Err("No task id provided"),
        }
    }
    fn parse_task_name(args: &mut impl Iterator<Item = String>) -> Result<String, &'static str> {
        match args.next() {
            Some(task_name) => {
                if task_name == "" {
                    return Err("Task name cannot be empty");
                }
                Ok(task_name)
            }
            None => Err("No task name provided"),
        }
    }
}
impl Command {
    fn parse_command(command_string: String) -> Result<Command, &'static str> {
        match command_string.as_str() {
            "add" => Ok(Command {
                kind: CommandKind::Modifying,
                name: Commands::Add,
            }),
            "update" => Ok(Command {
                kind: CommandKind::Modifying,
                name: Commands::Update,
            }),
            "delete" => Ok(Command {
                kind: CommandKind::Modifying,
                name: Commands::Delete,
            }),
            "mark-in-progress" => Ok(Command {
                kind: CommandKind::Modifying,
                name: Commands::MarkInProgress,
            }),
            "mark-done" => Ok(Command {
                kind: CommandKind::Modifying,
                name: Commands::MarkDone,
            }),
            "clear-done" => Ok(Command {
                kind: CommandKind::Modifying,
                name: Commands::ClearDone,
            }),
            "list-all" => Ok(Command {
                kind: CommandKind::Querying,
                name: Commands::ListAll,
            }),
            "list-done" => Ok(Command {
                kind: CommandKind::Querying,
                name: Commands::ListDone,
            }),
            "list-not-done" => Ok(Command {
                kind: CommandKind::Querying,
                name: Commands::ListNotDone,
            }),
            "list-progress" => Ok(Command {
                kind: CommandKind::Querying,
                name: Commands::ListProgress,
            }),
            _ => Err("Invalid command"),
        }
    }
}
