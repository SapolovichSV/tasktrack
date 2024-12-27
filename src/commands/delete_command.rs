pub struct DeleteCommand {
    config: Config,
    storage: Box<dyn ModifyStorage>,
}
