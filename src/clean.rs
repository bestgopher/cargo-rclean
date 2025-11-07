use cargo::core::Workspace;
use cargo::ops::{self as cargo_clean, CleanOptions};
use cargo::util::interning::InternedString;
use cargo::GlobalContext;
use crossbeam::channel::{unbounded, Sender};
use log::{error, info};
use std::path::PathBuf;
use std::thread::Builder;

/// num: the number of threads
pub fn clean(num: usize) -> (Sender<PathBuf>, impl FnOnce()) {
    let (sender, receiver) = unbounded::<PathBuf>();
    let mut handlers = Vec::with_capacity(num);

    for i in 0..num {
        let receiver = receiver.clone();

        let s = Builder::new()
            .name(format!("thread-{i}"))
            .spawn(move || {
                let config = GlobalContext::default().unwrap();
                let config_opt = CleanOptions {
                    gctx: &config,
                    spec: Default::default(),
                    targets: Default::default(),
                    profile_specified: Default::default(),
                    requested_profile: InternedString::from("dev"),
                    doc: Default::default(),
                    dry_run: false,
                };

                while let Ok(path) = receiver.recv() {
                    info!("clean: {}", path.display());
                    let w = Workspace::new(&path.join("Cargo.toml"), &config).unwrap();
                    if let Err(_e) = cargo_clean::clean(&w, &config_opt) {
                        error!("clean error");
                    }
                }
            })
            .expect("generate thread error");

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
