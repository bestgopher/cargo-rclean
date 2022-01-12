use std::collections::LinkedList;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Walk {
    list: LinkedList<PathBuf>,
}

impl Walk {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let mut s = Walk {
            list: LinkedList::new(),
        };
        if path.as_ref().to_path_buf().is_absolute() {
            s.list.push_front(path.as_ref().to_path_buf());
        } else {
            s.list.push_front(fs::canonicalize(path.as_ref()).unwrap());
        }
        s
    }
}

impl Iterator for Walk {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.list.pop_front() {
                Some(path) if path.is_dir() => {
                    if path.join("Cargo.toml").exists() {
                        return Some(path);
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

#[cfg(test)]
mod tests {
    use crate::walk::Walk;

    #[test]
    fn test_walk() {
        for i in Walk::new("../.") {
            println!("{:?}", i.as_path().display());
        }
    }
}
