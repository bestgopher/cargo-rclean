use std::path::PathBuf;
use std::env;

use clap::Parser;

/// A cargo subcommand which like `cargo clean` but clean all cargo projects.
#[derive(Parser, Debug)]
#[clap()]
pub struct Commands {
    /// Number of threads
    #[clap(short = 'n', long, default_value = "3")]
    pub thread_num: usize,

    /// Directory of clean, default: current directory
    #[clap(short = 'p', long, parse(try_from_str = parse_path))]
    pub path: PathBuf,

    /// Only clean these directories.If one in `excludes`, this one will be excluded.
    #[clap(long)]
    pub includes: Vec<PathBuf>,

    /// Exclude these directories.
    #[clap(long)]
    pub excludes: Vec<PathBuf>,
}

impl Commands {
    pub fn valid_path(&self, path: &PathBuf) -> bool {
        if !self.includes.is_empty() {
            if !self.includes.iter().any(|x| x.ends_with(path)) {
                return false;
            }
        }

        if !self.excludes.is_empty() {
            if self.excludes.iter().any(|x| x.ends_with(path)) {
                return false;
            }
        }

        true
    }
}

fn parse_path(p: &str) -> anyhow::Result<PathBuf> {
    if p.is_empty() {
        Ok(env::current_dir()?)
    } else {
        Ok(PathBuf::from(p))
    }
}