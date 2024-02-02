use anyhow::{bail, Context, Result};
use clap::Parser;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use std::fs;
use std::path::PathBuf;
use std::process;

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
    if let Err(err) = run().await {
        eprintln!("[ERROR] {}", err);
        process::exit(1);
    }
}

async fn run() -> Result<()> {
    let args = Cli::parse();

    let ollama = Ollama::default();

    // ollama list and verify if the model exists...
    let models_available = ollama
        .list_local_models()
        .await
        .with_context(|| "Unable to list local models")?;

    let model_exists = models_available
        .iter()
        .any(|model| model.name == args.model_name);

    if !model_exists {
        bail!("Model '{}' is not available.", args.model_name);
    }

    let content = fs::read_to_string(&args.document_path)
        .with_context(|| format!("Unable to read the file: {:?}", args.document_path))?;

    println!("[INFO] Scanning the document...");
    let res = ollama
        .generate(GenerationRequest::new(args.model_name, content))
        .await
        .map_err(|err| anyhow::anyhow!("Unable to generate the document: {}", err))?;

    println!("[INFO] Document scanned successfully");
    println!("[RESP]\n\n{}\n\n[ENDRESP]", res.response);

    Ok(())
}
