use super::Model;
use crate::entities::commands::Commands;
pub struct QueryModel {
    command: Commands,
}
impl Model for QueryModel {
    fn execute(&self) {
        println!("Querying {:?}", self.command);
    }
}
pub fn new(command: Commands) -> QueryModel {
    QueryModel { command }
}
