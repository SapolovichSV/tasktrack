enum Commands {
    Add,
    Update,
    Delete,
    ClearDone,
    ListAll,
    ListDone,
    ListNotDone,
    ListProgress,
}
enum CommandKind {
    Modifying,
    Querying,
}
pub struct Config {
    command: Command,
    task_id: Option<u8>,
    task_name: Option<String>,
}
struct Command {
    kind: CommandKind,
    name: Commands,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
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
                let task_id: u8 = match args.next() {
                    Some(task_id) => match task_id.parse() {
                        Ok(task_id) => task_id,
                        Err(_) => return Err("Error while try to parse task id"),
                    },
                    None => return Err("No task id provided"),
                };
                let task_name = match args.next() {
                    Some(task_name) => task_name,
                    None => return Err("No task name provided"),
                };
                Ok(Config {
                    command: command,
                    task_id: Some(task_id),
                    task_name: Some(task_name),
                })
            }
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
