#[derive(Debug)]
pub enum Commands {
    Add,
    Update,
    Delete,
    MarkInProgress,
    MarkDone,
    ClearDone,
    ListAll,
    ListDone,
    ListNotDone,
    ListProgress,
}
