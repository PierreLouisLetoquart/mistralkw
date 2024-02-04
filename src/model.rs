use std::error::Error;
use std::path::Path;
use std::result::Result;

use ollama_rs::models::create::CreateModelRequest;
use ollama_rs::models::LocalModel;
use ollama_rs::Ollama;

// Create a model using a modelfile
pub async fn create_model(
    ollama: &Ollama,
    name: &str,
    modelfile_path: &str,
) -> Result<(), Box<dyn Error>> {
    if !Path::new(modelfile_path).exists() {
        return Err("Modelfile path provided is wrong".into());
    }

    let req = CreateModelRequest::path(name.into(), modelfile_path.to_string());

    let _ = ollama
        .create_model(req)
        .await
        .map_err(|_| "An error occured during the model creation")?;

    Ok(())
}

// Get the list of local LLM installed on the system
pub async fn list_local_models(ollama: &Ollama) -> Result<Vec<LocalModel>, Box<dyn Error>> {
    let local_models = ollama.list_local_models().await?;
    Ok(local_models)
}

// Check if a certain model is available on the system
pub fn check_model_availability(
    name: &str,
    available: &[LocalModel],
) -> Result<(), Box<dyn Error>> {
    if !available.iter().any(|model| model.name == name) {
        return Err("Model isn't available".into());
    } else {
        Ok(())
    }
}
