use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use dirs::home_dir;
use serde::{Deserialize, Serialize};

pub const CONFIG_FILENAME: &str = "config.json";
pub const ROOT_FOLDER_NAME: &str = "WinterData";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum WinterMode {
    Local,
    Cloud,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppPaths {
    pub projects: String,
    pub uploads: String,
}

#[derive(Serialize, Deserialize,Debug, Clone)]
pub struct ModelDownloadInfo {
    pub model_name: String,
    pub quant: String,
    pub url: String,
    pub expected_sha256: String,
    pub estimated_size_bytes: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub mode: Option<WinterMode>,
    pub last_opened_project: Option<String>,
    pub recent_projects: Vec<String>,
    pub paths: AppPaths,
    pub model_file_size_estimate: Option<u64>,
    pub cached_model_info: Option<ModelDownloadInfo>,
}

pub fn setup_internal_dirs() -> std::io::Result<PathBuf> {
    let base_path = home_dir()
        .map(|h| h.join(ROOT_FOLDER_NAME))
        .expect("Cannot determine home directory");

    let projects_dir = base_path.join("projects");
    let uploads_dir = base_path.join("assistant/uploads");
    let config_path = base_path.join(CONFIG_FILENAME);

    fs::create_dir_all(&projects_dir)?;
    fs::create_dir_all(&uploads_dir)?;

    if !config_path.exists() {
        let config = AppConfig {
            mode: None,
            last_opened_project: None,
            recent_projects: vec![],
            paths: AppPaths {
                projects: projects_dir.to_string_lossy().to_string(),
                uploads: uploads_dir.to_string_lossy().to_string(),
            },
            model_file_size_estimate:None,
            cached_model_info: None
        };

        let json = serde_json::to_string_pretty(&config)?;
        let mut file = File::create(&config_path)?;
        file.write_all(json.as_bytes())?;
    }

    println!("[tauri] ✅ Internal config initialized at: {}", config_path.display());

    Ok(base_path)
}

pub fn load_config() -> std::io::Result<AppConfig> {
    let config_path = home_dir()
        .expect("Could not get home dir")
        .join(ROOT_FOLDER_NAME)
        .join(CONFIG_FILENAME);

    let contents = fs::read_to_string(&config_path)?;
    let config: AppConfig = serde_json::from_str(&contents)?;
    Ok(config)
}

pub fn set_last_opened_project(project_path: String) -> std::io::Result<()> {
    let mut config = load_config()?;
    config.last_opened_project = Some(project_path.clone());

    if let Some(pos) = config.recent_projects.iter().position(|p| p == &project_path) {
        config.recent_projects.remove(pos);
    }

    config.recent_projects.insert(0, project_path);
    config.recent_projects.truncate(10);

    let config_path = home_dir()
        .expect("Could not get home dir")
        .join(ROOT_FOLDER_NAME)
        .join(CONFIG_FILENAME);

    let json = serde_json::to_string_pretty(&config)?;
    fs::write(config_path, json)?;
    Ok(())
}
pub fn save_config(config: &AppConfig)->std::io::Result<()>{
    let config_path = home_dir()
        .expect("Could not get home dir")
        .join(ROOT_FOLDER_NAME)
        .join(CONFIG_FILENAME);
    let json = serde_json::to_string_pretty(config)?;
    fs::write(config_path,json)?;
    Ok(())
}