mod walk;
mod clean;
mod command;

use log::{debug, error, LevelFilter};
use clap::Parser;

use crate::walk::Walk;
use crate::command::Commands;

fn main() -> anyhow::Result<()> {
    let command: Commands = Commands::parse();
    log_init(command.debug);

    debug!("command line arguments: {}", command);
    let (sender, wait_func) = clean::clean(command.thread_num);

    for path in Walk::new(&command) {
        if let Err(_err) = sender.send(path) {
            error!("send error");
        }
    }

    drop(sender);

    wait_func();

    Ok(())
}

fn log_init(debug: bool) {
    let mut builder = env_logger::Builder::new();
    if debug {
        builder.filter_level(LevelFilter::Debug);
    }

    builder.filter_module(" cargo_rclean::clean", LevelFilter::Debug).init();
}