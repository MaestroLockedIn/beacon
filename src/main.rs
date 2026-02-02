use std::{fs::File, sync::Arc};

use crate::config::Config;

mod config;
mod scheduler;
mod server;

#[tokio::main]
async fn main() {
    // TODO: Read yaml file, serialize to a struct and print json
    let file = File::open("config.yaml").expect("Unable to open file");
    let config: Config = serde_yaml::from_reader(file).expect("Could not serialize file");

    println!("Configuration loaded successfully: {:#?}", config);
    let config = Arc::new(config);
    tokio::spawn(async {
        scheduler::run_tasks(config).await;
    });

    server::start().await;
    println!("Shutting down...");
}
