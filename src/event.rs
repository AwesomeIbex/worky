use std::path::PathBuf;

mod shell;

#[tokio::main]
pub async fn start_tokio(path_rx: std::sync::mpsc::Receiver<PathBuf>) {
    while let Ok(event) = path_rx.recv() {
        println!("Received event in loop {:?}", event);
        let meta = event.metadata();
        // TODO atm this runs twice
        if let Ok(metadata) = meta {
            if metadata.is_file() {
                shell::run_file(event.to_str().unwrap()).unwrap().for_each(|stdout_line| println!("{}", stdout_line));
            } else if metadata.is_dir() {
                shell::run_directory(event.to_str().unwrap()).unwrap().for_each(|stdout_line| println!("{}", stdout_line));
            }
        }
    };
}