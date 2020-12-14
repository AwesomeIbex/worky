use std::sync::mpsc::channel;
use notify::{watcher, RecursiveMode, Watcher, DebouncedEvent};
use std::time::Duration;
use crate::cli::Opts;
use std::fs;
use rand::Rng;
use rand::distributions::Alphanumeric;
use std::path::PathBuf;

mod cli;
mod event;

///
/// User will pass in a command file, this could be a raw file or a location to the file
///
/// cli could create a script from some yaml file, but for now we will just expect the script to be in the FS OR passed in raw
/// cli will read file and determine some things
/// the type of job
/// the directory to store updates and dispatch
///
fn main() {
    pretty_env_logger::init();

    let cli_opts = cli::get_opts_args();
    let mut worker_prefix = String::from("WORKER_");
    let worker_id = rand::thread_rng().sample_iter(&Alphanumeric).take(20).collect::<String>();
    worker_prefix.push_str(&worker_id);
    let mut path = cli_opts.jobs_path.to_str().unwrap().to_string();
    if path.ends_with("/") {
        path.push_str(&worker_prefix);
    } else {
        path.push_str("/");
        path.push_str(&worker_prefix);
    }
    fs::create_dir_all(&path);

    // file events
    let (watcher_tx, watcher_rx) = channel();
    let mut watcher = watcher(watcher_tx, Duration::from_millis(300)).unwrap(); //TODO test delay
    watcher.watch(&path, RecursiveMode::Recursive).unwrap();

    // tokio
    let (tokio_tx, tokio_rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        event::start_tokio(tokio_rx);
    });

    loop {
        match watcher_rx.recv() {
            Ok(event) => {
                match event {
                    // new job gets written to workers directory
                    // worker receives event
                    DebouncedEvent::Create(dir) => {
                        println!("File created at {:?}", dir.to_str());
                        let mut dir_string = dir.to_str().unwrap().to_string();
                        let mut started_prefix = String::from("_STARTED");
                        dir_string.push_str(&started_prefix);

                        // updates postfix to show its started
                        let started_path = PathBuf::from(&dir_string);
                        fs::rename(dir, &started_path);
                        // worker sends path to tokio loop to handle
                        tokio_tx.send(started_path);
                        // waits for completion
                        // updates postfix to show finished
                        // exits the program
                    }
                    DebouncedEvent::Error(_, _) => {} // Log
                    _ => {} // Log
                }
            },
            Err(e) => { println!("Error {}", e) },
        }
    };
    // fs::remove_dir_all(&path);
}