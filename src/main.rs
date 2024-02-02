use ollama_rs::{generation::completion::request::GenerationRequest, models::LocalModel, Ollama};
use std::fs;
use std::process;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    model_name: String,

    #[arg(short, long, value_name = "FILE")]
    document_path: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let ollama = Ollama::default();

    // ollama list and verify if the model exists...
    let models_available: Vec<LocalModel> =
        ollama.list_local_models().await.unwrap_or_else(|err| {
            eprintln!("[ERROR] Unable to list local models: {}", err);
            process::exit(1);
        });

    let model_exists = models_available
        .iter()
        .any(|model| model.name == args.model_name);

    if !model_exists {
        eprintln!("[ERROR] Model '{}' is not available.", args.model_name);
        process::exit(1);
    }

    let content = fs::read_to_string(args.document_path).unwrap_or_else(|err| {
        eprintln!("[ERROR] Unable to read the file: {}", err);
        process::exit(1);
    });

    println!("[INFO] Scanning the document...");
    let res = ollama
        .generate(GenerationRequest::new(args.model_name, content))
        .await;

    if let Ok(res) = res {
        println!("[INFO] Document scanned successfuly");
        println!("[RESP]\n\n{}\n\n[ENDRESP]", res.response);
    } else {
        eprintln!("[ERROR] An error occured during the scan");
        process::exit(1);
    }
}
