use std::path::PathBuf;

#[tokio::main]
pub async fn start_tokio(path_rx: std::sync::mpsc::Receiver<PathBuf>) {
    while let Ok(event) = path_rx.recv() {
        println!("Received event in loop {:?}", event);
    };
}