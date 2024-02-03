use std::result::Result;
use std::error::Error;

use ollama_rs::models::LocalModel;
use ollama_rs::Ollama;

// Get the list of local LLM installed on the system
async fn list_local_models(ollama: &Ollama) -> Result<Vec<LocalModel>, Box<dyn Error>> {
    let local_models = ollama.list_local_models().await?;
    Ok(local_models)
}

// Check if a certain model is available on the system
fn check_model_availability(name: &str, available: &[LocalModel]) -> Result<(), Box<dyn Error>> {
    if !available.iter().any(|model| model.name == name) {
        return Err("Model isn't available".into());
    } else {
        Ok(())
    }
}
