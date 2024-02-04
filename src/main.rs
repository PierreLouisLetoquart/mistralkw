use mixtralkw::Args;
use mixtralkw::{generation, model};

use clap::Parser;
use ollama_rs::Ollama;
use std::fs;
use std::path::Path;
use std::process::exit;

static MODELFILE: &str = "./assets/Modelfile";

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if !Path::new(&args.document).exists() {
        eprintln!("[ERROR] Document path provided is wrong");
        exit(1);
    }

    let ollama = Ollama::default();

    let models = model::list_local_models(&ollama).await.unwrap();

    if !model::check_model_availability(&args.model, &models).is_ok() {
        let _ = model::create_model(&ollama, &args.model, MODELFILE).await;
    }

    let content = fs::read_to_string(&args.document).unwrap();

    let res = generation::gen_keywords(&ollama, &args.model, &content)
        .await
        .unwrap();

    match &args.output {
        Some(path) => {
            fs::write(&path, res).unwrap();
        }
        None => println!("{}", res),
    }
}
