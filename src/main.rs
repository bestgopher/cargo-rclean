mod walk;
mod clean;

use std::env;
use log::error;
use crate::walk::Walk;

fn main() -> anyhow::Result<()> {
    let base_dir = env::current_dir()?;
    let (sender, wait_func) = clean::clean(3);

    for path in Walk::new(base_dir) {
        if let Err(_err) = sender.send(path) {
            error!("send error");
        }
    }

    drop(sender);

    wait_func();

    Ok(())
}
