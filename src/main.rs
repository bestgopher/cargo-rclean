mod walk;
mod clean;
mod command;

use log::error;
use clap::Parser;

use crate::walk::Walk;
use crate::command::Commands;

fn main() -> anyhow::Result<()> {
    let command: Commands = Commands::parse();
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
