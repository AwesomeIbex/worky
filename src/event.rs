use std::path::PathBuf;
use std::io::{Write, IoSlice, LineWriter};

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
                let mut log_file = event_path.clone().to_str().unwrap().to_string();
                log_file.push_str("/");
                log_file.push_str("logs");
                let mut file = std::fs::File::create(log_file).unwrap();
                let mut file = LineWriter::new(file);

                shell::run_directory(event_path.to_str().unwrap()).unwrap().for_each(|line| {
                    let mut line = line;
                    line.push_str("\n");
                    file.write(line.as_bytes());
                })
            }
        }
    };
}