use sysinfo::{System};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ModelChoice {
    pub model_name: String,
    pub quant_level: String,
    pub context_size: usize,
    pub estimated_download_gb: f32,
}

#[tauri::command]
pub fn pick_optimal_model() -> ModelChoice {
    let mut sys = System::new_all();
    sys.refresh_memory();

    let ram_gb = sys.total_memory() / 1024 / 1024;

    if ram_gb >= 12 {
        ModelChoice {
            model_name: "mistral".to_string(),
            quant_level: "Q4_K_M".to_string(),
            context_size: 4096,
            estimated_download_gb: 4.3,
        }
    } else if ram_gb >= 8 {
        ModelChoice {
            model_name: "phi".to_string(),
            quant_level: "Q4_0".to_string(),
            context_size: 2048,
            estimated_download_gb: 1.2,
        }
    } else {
        ModelChoice {
            model_name: "tinyllama".to_string(),
            quant_level: "Q2_K".to_string(),
            context_size: 1024,
            estimated_download_gb: 0.5,
        }
    }
}