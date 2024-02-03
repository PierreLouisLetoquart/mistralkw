use anyhow::{bail, Context, Result};
use clap::Parser;
use ollama_rs::generation::format::FormatType;
use ollama_rs::{generation::completion::request::GenerationRequest, models::LocalModel, Ollama};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Keywords Generator")]
#[command(author = "Pierre-Louis L. <randlgint@proton.me>")]
#[command(version = "1.0")]
#[command(
    about = "Generates keywords from a document using an Ollama model.",
    long_about = None
)]
pub struct Cli {
    #[arg(
        short,
        long,
        value_name = "MODEL NAME",
        help = "The model to use. Use 'ollama list' to list available models."
    )]
    model: String,

    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "The document to generate keywords from."
    )]
    document: PathBuf,

    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Output file, if not keywords are printed to stdout."
    )]
    output: Option<PathBuf>,
}

pub async fn gen_keywords(args: &Cli, ollama: &Ollama) -> Result<()> {
    let models_available = list_local_models(&ollama).await?;

    check_model_availability(&args.model, &models_available)?;

    let content = fs::read_to_string(&args.document)
        .with_context(|| format!("Unable to read the file: {:?}", &args.document))?;

    println!("Generating keywords...");

    let res = ollama
        .generate(GenerationRequest::new(args.model.to_string(), content).format(FormatType::Json))
        .await
        .map_err(|err| anyhow::anyhow!("Unable to generate the document: {}", err))?;

    if let Some(output) = &args.output {
        fs::write(output, &res.response)
            .with_context(|| format!("Unable to write to the file: {:?}", output))?;
        println!("Keywords generated and written to {:?}", output);
    } else {
        println!("{}", &res.response);
    }

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
            model: "test".to_string(),
            document: PathBuf::from("test.txt"),
            output: None,
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
