use std::path::PathBuf;
use std::thread::Builder;
use cargo::Config;
use cargo::core::Workspace;
use cargo::ops::{self as cargo_clean, CleanOptions};
use cargo::util::interning::InternedString;
use crossbeam::channel::{Sender, unbounded};
use log::error;

/// num: the number of threads
pub fn clean(num: usize) -> (Sender<PathBuf>, impl FnOnce()) {
    let (sender, receiver) = unbounded::<PathBuf>();
    let mut handlers = Vec::with_capacity(num);

    for i in 0..num {
        let receiver = receiver.clone();

        let s = Builder::new().name(format!("thread-{}", i)).spawn(
            move || {
                let config = Config::default().unwrap();
                let config_opt = CleanOptions {
                    config: &config,
                    spec: Default::default(),
                    targets: Default::default(),
                    profile_specified: Default::default(),
                    requested_profile: InternedString::from("dev"),
                    doc: Default::default(),
                };

                while let Ok(path) = receiver.recv() {
                    let w = Workspace::new(path.join("Cargo.toml").as_path(), &config).unwrap();
                    if let Err(_e) = cargo_clean::clean(&w, &config_opt) {
                        error!("clean error");
                    }
                }
            }
        ).expect("generate thread error");

        handlers.push(s);
    }

    let f = move || {
        for i in handlers {
            let thread_name = i.thread().name().unwrap_or("unknown").to_string();
            if let Err(_err) = i.join() {
                error!("thread `{}` join err", thread_name);
            }
        }
    };

    (sender, f)
}