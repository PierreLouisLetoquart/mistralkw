use anyhow::{bail, Context, Result};
use clap::Parser;
use ollama_rs::{generation::completion::request::GenerationRequest, models::LocalModel, Ollama};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    model_name: String,

    #[arg(short, long, value_name = "FILE")]
    document_path: PathBuf,
}

pub async fn gen_keywords(args: &Cli, ollama: &Ollama) -> Result<()> {
    let models_available = list_local_models(&ollama).await?;

    check_model_availability(&args.model_name, &models_available)?;

    let content = fs::read_to_string(&args.document_path)
        .with_context(|| format!("Unable to read the file: {:?}", &args.document_path))?;

    println!("Generating keywords...");

    let res = ollama
        .generate(GenerationRequest::new(args.model_name.to_string(), content))
        .await
        .map_err(|err| anyhow::anyhow!("Unable to generate the document: {}", err))?;

    println!("{}", res.response);

    Ok(())
}

async fn list_local_models(ollama: &Ollama) -> Result<Vec<LocalModel>> {
    ollama
        .list_local_models()
        .await
        .with_context(|| "Unable to list local models")
}

fn check_model_availability(model_name: &str, models_available: &[LocalModel]) -> Result<()> {
    if !models_available
        .iter()
        .any(|model| model.name == model_name)
    {
        bail!("Model '{}' is not available.", model_name);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gen_keywords() {
        let args = Cli {
            model_name: "test".to_string(),
            document_path: PathBuf::from("test.txt"),
        };
        let ollama = Ollama::default();

        let res = gen_keywords(&args, &ollama).await;
        assert!(res.is_err());
    }

    #[test]
    fn test_check_model_availability() {
        let models = vec![
            LocalModel {
                name: "test".to_string(),
                modified_at: "".to_string(),
                size: 0,
            },
            LocalModel {
                name: "test2".to_string(),
                modified_at: "".to_string(),
                size: 0,
            },
        ];

        let res = check_model_availability("test", &models);
        assert!(res.is_ok());

        let res = check_model_availability("test3", &models);
        assert!(res.is_err());
    }
}
