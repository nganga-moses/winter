use std::{
    fs,
    io::{Error},
    path::PathBuf,
};

use crate::config::{AppConfig, WinterMode, CONFIG_FILENAME, ROOT_FOLDER_NAME};
use dirs::home_dir;

fn config_path() -> PathBuf {
    home_dir()
        .expect("Could not get home dir")
        .join(ROOT_FOLDER_NAME)
        .join(CONFIG_FILENAME)
}

fn load_config_file() -> Result<AppConfig, Error> {
    let contents = fs::read_to_string(config_path())?;
    let config: AppConfig = serde_json::from_str(&contents)?;
    Ok(config)
}

fn save_config_file(config: &AppConfig) -> Result<(), Error> {
    let json = serde_json::to_string_pretty(config)?;
    fs::write(config_path(), json)?;
    Ok(())
}

pub fn get_mode() -> Result<Option<WinterMode>, Error> {
    let config = load_config_file()?;
    Ok(config.mode)
}

pub fn set_mode(mode: WinterMode) -> Result<(), Error> {
    let mut config = load_config_file()?;
    config.mode = Some(mode);
    save_config_file(&config)
}

#[tauri::command]
pub fn get_current_mode() -> Result<Option<WinterMode>, String> {
    get_mode().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_current_mode(mode: WinterMode) -> Result<(), String> {
    set_mode(mode).map_err(|e| e.to_string())
}