// Simple script to download the fastembed model
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Downloading AllMiniLML6V2 model...");
    let mut opts =
        InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true);
    opts = opts.with_cache_dir(std::path::PathBuf::from("models"));

    let mut attempts = 0;
    let max_attempts = 5;
    let mut delay = Duration::from_secs(2);

    loop {
        attempts += 1;
        match TextEmbedding::try_new(opts.clone()) {
            Ok(_) => {
                println!("Model downloaded successfully!");
                break;
            }
            Err(e) => {
                if attempts >= max_attempts {
                    panic!(
                        "Failed to download model after {} attempts: {}",
                        max_attempts, e
                    );
                }
                println!(
                    "Attempt {} failed: {}. Retrying in {:?}...",
                    attempts, e, delay
                );
                thread::sleep(delay);
                delay *= 2;
            }
        }
    }
}
