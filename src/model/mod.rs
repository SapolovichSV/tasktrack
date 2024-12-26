use crate::{
    config::Config,
    entities::command_kind::{self, CommandKind},
};
mod entities;
mod modify_model;
mod query_model;
mod storage;
pub trait Model {
    fn execute(&self);
}
pub trait ModelFactory {
    fn create_model(&self, config: Config) -> Box<dyn Model>;
}
pub fn get_factory(query_type: &command_kind::CommandKind) -> Box<dyn ModelFactory> {
    match query_type {
        CommandKind::Querying => Box::new(QueryFactory),
        CommandKind::Modifying => Box::new(ModifyFactory),
    }
}

struct QueryFactory;
impl ModelFactory for QueryFactory {
    fn create_model(&self, config: Config) -> Box<dyn Model> {
        Box::new(query_model::new(config.command.name))
    }
}

struct ModifyFactory;
impl ModelFactory for ModifyFactory {
    fn create_model(&self, config: Config) -> Box<dyn Model> {
        Box::new(modify_model::new(config))
    }
}