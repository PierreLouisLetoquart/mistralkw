use ollama_rs::{generation::completion::request::GenerationRequest, models::LocalModel, Ollama};
use std::env;
use std::fs;
use std::process;

fn help(program: &str) {
    eprintln!("[ERROR] This program required 2 args");
    println!("[USAGE] {} <model-name> <document-path>", program);
    process::exit(1);
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => println!(
            "[INFO] Scanning {} using the {} model",
            &args[2], &args[1]
        ),
        _ => help(&args[0]),
    }

    let model_name = &args[1];
    let file_path = &args[2];

    let ollama = Ollama::default();

    // ollama list and verify if the model exists...
    let models_available: Vec<LocalModel> = ollama.list_local_models().await.unwrap_or_else(|err| {
        eprintln!("[ERROR] Unable to list local models: {}", err);
        process::exit(1);
    });

    let model_exists = models_available.iter().any(|model| model.name == model_name.to_string());

    if !model_exists {
        eprintln!("[ERROR] Model '{}' is not available.", model_name);
        process::exit(1);
    }

    let content = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("[ERROR] Unable to read the file: {}", err);
        process::exit(1);
    });

    println!("[INFO] Scanning the document...");
    let res = ollama
        .generate(GenerationRequest::new(model_name.into(), content))
        .await;

    if let Ok(res) = res {
        println!("[INFO] Document scanned successfuly");
        println!("[RESP]\n\n{}\n\n[ENDRESP]", res.response);
    } else {
        eprintln!("[ERROR] An error occured during the scan");
        process::exit(1);
    }
}
