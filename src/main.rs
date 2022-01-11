mod walk;

use cargo::ops::{clean, CleanOptions};
use cargo::core::Workspace;
use cargo::util::{Config, interning::InternedString};


use crate::walk::Walk;

fn main() -> anyhow::Result<()> {
    for path in Walk::new(".") {
        let config = &Config::default()?;
        let w = Workspace::new(path.join("Cargo.toml").as_path(), config)?;
        let config_opt: CleanOptions = CleanOptions {
            config: &config,
            spec: Default::default(),
            targets: Default::default(),
            profile_specified: Default::default(),
            requested_profile: InternedString::from("dev"),
            doc: Default::default(),
        };

        clean(&w, &config_opt)?;
    }

    Ok(())
}
