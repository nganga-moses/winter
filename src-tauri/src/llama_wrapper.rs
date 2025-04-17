use std::process::{Command, Stdio};
use std::path::Path;

use crate::model_selector::pick_optimal_model;

#[tauri::command]
pub fn run_llama_inference(prompt: String) -> Result<String, String> {
    let config = pick_optimal_model();

    let model_path = format!(
        "models/{}/{}.gguf",
        config.model_name,
        config.quant_level
    );

    if !Path::new(&model_path).exists() {
        return Err(format!("Model not found: {}", model_path));
    }

    let output = Command::new("./bin/llama.cpp/main")
        .args(&[
            "-m", &model_path,
            "-p", &prompt,
            "-c", &config.context_size.to_string(),
            "--temp", "0.7",
            "--repeat_penalty", "1.1",
        ])
        .stdout(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to run llama.cpp: {}", e))?;

    let response = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(response)
}