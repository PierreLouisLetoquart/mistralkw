use clap::Parser;
use mixtralkw::{gen_keywords, Cli};
use ollama_rs::Ollama;
use std::process;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let ollama = Ollama::default();

    if let Err(err) = gen_keywords(&args, &ollama).await {
        eprintln!("[ERROR] {}", err);
        process::exit(1);
    }
}
