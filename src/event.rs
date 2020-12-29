use std::io::{IoSlice, LineWriter, Write};
use std::path::PathBuf;

mod shell;

#[tokio::main]
pub async fn start_tokio(path_rx: std::sync::mpsc::Receiver<PathBuf>) {
    while let Ok(event_path) = path_rx.recv() {
        println!("Received event in loop {:?}", event_path);

        let meta = event_path.metadata();
        // TODO atm this runs twice
        if let Ok(metadata) = meta {
            if metadata.is_file() {
                // shell::run_file(event_path.to_str().unwrap()).unwrap()
                //     .for_each(|line| {
                //         file.write_all(line.as_bytes());
                //     });
                println!("Im a file");
            } else if metadata.is_dir() {
                let dir = event_path.to_str().expect("Failed to write event path to a string");
                shell::run_directory(dir).expect("Failed to run script in directory");
            }
        }
    };
}