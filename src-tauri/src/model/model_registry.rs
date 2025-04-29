use crate::config::{AppConfig, AppPaths, load_config, save_config,ModelDownloadInfo};

#[tauri::command]
pub fn get_model_download_info() -> ModelDownloadInfo {
    let choice = crate::model::model_selector::pick_optimal_model();

    let info = match choice.model_name.as_str() {
        "mistral" => ModelDownloadInfo {
            model_name: "mistral".into(),
            quant: choice.quant_level,
            url: "https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF/resolve/main/mistral-7b-instruct-v0.2.Q4_K_M.gguf".into(),
            expected_sha256: "3e0039fd0273fcbebb49228943b17831aadd55cbcbf56f0af00499be2040ccf9".into(),
            estimated_size_bytes: 4_370_000_000, //4.37 GB
        },
        "phi" => ModelDownloadInfo {
            model_name: "phi".into(),
            quant: choice.quant_level,
            url: "https://huggingface.co/TheBloke/phi-2-GGUF/resolve/main/phi-2.Q4_K_M.gguf".into(),
            expected_sha256: "324356668fa5ba9f4135de348447bb2bbe2467eaa1b8fcfb53719de62fbd2499".into(),
            estimated_size_bytes: 1_790_000_000, //1.79GB
        },
        _ => ModelDownloadInfo {
            model_name: "tinyllama".into(),
            quant: choice.quant_level,
            url: "https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF/resolve/main/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf".into(),
            expected_sha256: "9fecc3b3cd76bba89d504f29b616eedf7da85b96540e490ca5824d3f7d2776a0".into(),
            estimated_size_bytes: 669_000_000, //660MB
        },
    };
    let mut config = load_config().unwrap_or_else(|_| AppConfig{
        mode: None,
        last_opened_project: None,
        recent_projects: vec![],
        paths: AppPaths{
            projects: "".into(),
            uploads:"".into(),
        },
        model_file_size_estimate: None,
        cached_model_info: None
    });

    config.model_file_size_estimate = Some(info.estimated_size_bytes);
    config.cached_model_info = Some(info.clone());
    let _= save_config(&config); // dont fail startup if this fails
    info
}
