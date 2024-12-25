//use config::Config;
use std::env;
mod config;
mod entities;
mod model;
fn main() {
    let config = config::Config::build(env::args()).unwrap_or_else(|x| {
        eprintln!("Error: {}", x);
        std::process::exit(1)
    });
    println!("{:?}", config);
    let factory = model::get_factory(&config.command.kind);
    let model = factory.create_model(config);
    model.execute();
}
