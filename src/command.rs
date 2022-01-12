use std::path::{Path, PathBuf};
use std::env;
use std::fmt::Display;

use clap::Parser;
use itertools::Itertools;

/// A cargo subcommand which like `cargo clean` but clean all cargo projects.
#[derive(Parser, Debug, Default)]
#[clap(version = "0.1.0")]
pub struct Commands {
    /// Number of threads
    #[clap(short = 'n', long, default_value = "3")]
    pub thread_num: usize,

    /// Directory of clean, default: current directory
    #[clap(short = 'p', long, parse(try_from_str = parse_path), default_value = ".")]
    pub path: PathBuf,

    /// Only clean these directories.If one in `excludes`, this one will be excluded.
    #[clap(long)]
    pub includes: Vec<PathBuf>,

    /// Exclude these directories.
    #[clap(long)]
    pub excludes: Vec<PathBuf>,

    /// Output Debug logs.
    #[clap(long)]
    pub debug: bool,
}

impl Commands {
    pub fn valid_path(&self, path: &Path) -> bool {
        if !self.includes.is_empty() && !self.includes.iter().any(|x| path.ends_with(x)) {
            return false;
        }

        if !self.excludes.is_empty() && self.excludes.iter().any(|x| path.ends_with(x)) {
            return false;
        }

        true
    }
}

impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "thread_num: {}, path: {}, includes: [{}], excludes: [{}], debug: {}",
            self.thread_num,
            self.path.display(),
            self.includes.iter().map(|x| x.display().to_string()).join(", "),
            self.excludes.iter().map(|x| x.display().to_string()).join(", "),
            self.debug
        )
    }
}

fn parse_path(p: &str) -> anyhow::Result<PathBuf> {
    if p.is_empty() {
        Ok(env::current_dir()?)
    } else {
        Ok(PathBuf::from(p).canonicalize()?)
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::PathBuf;

    use crate::command::parse_path;
    use crate::Commands;

    #[test]
    fn test_parse_path() {
        assert_eq!(parse_path("").unwrap(), env::current_dir().unwrap());
        println!("{}", parse_path(".").unwrap().display());
        assert!(parse_path(".").unwrap().starts_with("/"));
    }

    #[test]
    fn test_display() {
        assert_eq!(
            "thread_num: 3, path: /root, includes: [/user, /home], excludes: [/ect, /mnt], debug: true",
            format!(
                "{}",
                Commands {
                    thread_num: 3,
                    path: PathBuf::from("/root"),
                    includes: vec![PathBuf::from("/user"), PathBuf::from("/home")],
                    excludes: vec![PathBuf::from("/ect"), PathBuf::from("/mnt")],
                    debug: true,
                }
            ).as_str()
        )
    }

    #[test]
    fn test_valid_path() {
        let command = Commands {
            includes: vec![PathBuf::from("a")],
            ..Default::default()
        };

        assert!(command.valid_path(&PathBuf::from("/user/a")));
        assert!(!command.valid_path(&PathBuf::from("/user/b")));
    }
}
