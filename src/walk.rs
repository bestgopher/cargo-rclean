use std::collections::LinkedList;

use std::path::{PathBuf};

use crate::command::Commands;

pub struct Walk<'a> {
    list: LinkedList<PathBuf>,
    command: &'a Commands,
}

impl<'a> Walk<'a> {
    pub fn new(command: &'a Commands) -> Self {
        let mut list = LinkedList::new();

        list.push_back(command.path.clone());

        Walk { list, command }
    }
}

impl<'a> Iterator for Walk<'a> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.list.pop_front() {
                Some(path) if path.is_dir() => {
                    if path.join("Cargo.toml").exists() {
                        if self.command.valid_path(&path) {
                            return Some(path);
                        }
                    } else if let Ok(f) = path.read_dir() {
                        self.list.extend(
                            f.filter(|x| x.is_ok() && x.as_ref().unwrap().path().is_dir())
                                .map(|x| x.unwrap().path()),
                        )
                    }
                }
                Some(_) => continue,
                None => break,
            }
        }

        None
    }
}
