use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use std::env;
use std::fs;
use std::process;

fn help(program: &str) {
    eprint!("[ERROR] : {} required 2 args", program);
    println!("[USAGE] : {} <model-name> <document-path>", program);
    process::exit(1);
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => help(&args[0]),
        2 => println!(
            "[INFO] : scanning {} using the {} model",
            &args[2], &args[1]
        ),
        _ => help(&args[0]),
    }

    let model = &args[1];
    let file_path = &args[2];

    let content = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("[ERROR] : Unable to read the file: {}", err);
        process::exit(1);
    });

    let ollama = Ollama::default();

    println!("[INFO] : Scanning the document...");
    let res = ollama
        .generate(GenerationRequest::new(model.into(), content))
        .await;

    if let Ok(res) = res {
        println!("[INFO] : Document scanned successfuly");
        println!("[RESP] : \n{}", res.response);
    } else {
        eprintln!("[ERROR] : An error occured during the scan");
        process::exit(1);
    }
}
