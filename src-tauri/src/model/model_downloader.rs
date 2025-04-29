use std::{
    fs::OpenOptions,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, OnceLock,
    },
};

use reqwest::header::{CONTENT_LENGTH, RANGE};
use sha2::{Digest, Sha256};
use tokio::{fs::{self, File, read}, io::AsyncWriteExt};
use tauri::{AppHandle};
use dirs::home_dir;
use tauri::Emitter;
use futures_util::stream::StreamExt;
use crate::config::{load_config};
use serde::Serialize;

static CANCEL_DOWNLOAD: OnceLock<Arc<AtomicBool>> = OnceLock::new();

#[tauri::command]
pub async fn download_model_file(app: AppHandle) -> Result<String, String> {
    // Load model info from config
    let config = load_config().map_err(|e| format!("Config error: {}", e))?;
    let info = config.cached_model_info.ok_or("Missing model info in config")?;

    let base_path = home_dir()
        .ok_or("Unable to find home dir")?
        .join("WinterData/models")
        .join(&info.model_name);

    fs::create_dir_all(&base_path)
        .await
        .map_err(|e| format!("Directory error: {}", e))?;

    let filename = format!("{}.gguf", info.quant);
    let file_path = base_path.join(&filename);
    let tmp_path = base_path.join(format!("{}.partial", &filename));

    let client = reqwest::Client::new();

    let resume_from = if tmp_path.exists() {
        tmp_path.metadata().ok().map(|m| m.len()).unwrap_or(0)
    } else {
        0
    };

    let mut file = File::from_std(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&tmp_path)
            .map_err(|e| format!("File error: {}", e))?,
    );

    let mut request = client.get(&info.url);
    if resume_from > 0 {
        request = request.header(RANGE, format!("bytes={}-", resume_from));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Download error: {}", e))?;

    let total_size = response
        .headers()
        .get(CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .map(|len| len + resume_from)
        .unwrap_or(0);

    let cancel_flag = CANCEL_DOWNLOAD.get_or_init(|| Arc::new(AtomicBool::new(false))).clone();
    let mut stream = response.bytes_stream();
    let mut downloaded = resume_from;
    let start_time = std::time::Instant::now();

    while let Some(chunk) = stream.next().await {
        if cancel_flag.load(Ordering::Relaxed) {
            return Err("Download cancelled.".into());
        }

        let chunk = chunk.map_err(|e| format!("Stream error: {}", e))?;
        file.write_all(&chunk)
            .await
            .map_err(|e| format!("Write error: {}", e))?;
        downloaded += chunk.len() as u64;

        #[derive(Serialize, Clone)]
        struct DownloadProgress {
            downloaded: u64,
            total: u64,
            percent: u8,
            speed_bytes_per_sec: u64,
            eta_seconds: u64,
        }

        let percent = ((downloaded as f64 / total_size as f64) * 100.0).floor() as u8;

        // Human-readable string like "842.12 MB / 1.92 GB"
        fn format_bytes(bytes: u64) -> String{
            let gb = 1_000_000_000_f64;
            let mb = 1_000_000_f64;
            if bytes as f64 > gb {
                format!("{:.2} GB", bytes as f64 / gb)
            } else {
                format!("{:.2} MB", bytes as f64 / mb)
            }
        }

        let elapsed_secs = start_time.elapsed().as_secs().max(1);
        let speed = downloaded / elapsed_secs;
        let remaining = total_size.saturating_sub(downloaded);
        let eta = if speed > 0 {
            remaining / speed
        } else {
            0
        };

        let progress = DownloadProgress {
            downloaded: downloaded,
            total: total_size,
            percent,
            speed_bytes_per_sec: speed,
            eta_seconds: eta,
        };

        let _ = app.emit("model-download-progress", progress);

    }

    file.flush().await.map_err(|e| e.to_string())?;
    drop(file);

    fs::rename(&tmp_path, &file_path)
        .await
        .map_err(|e| format!("Rename failed: {}", e))?;

    let _ = fs::remove_file(&tmp_path).await;
    verify_checksum(&file_path, &info.expected_sha256).await?;

    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn estimate_model_file_size(url: String) -> Result<u64, String> {
    let client = reqwest::Client::new();
    let resp = client
        .head(&url)
        .send()
        .await
        .map_err(|e| format!("HEAD error: {}", e))?;

    let content_length = resp
        .headers()
        .get(CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok());

    content_length.ok_or_else(|| "Could not determine file size".into())
}

#[tauri::command]
pub fn cancel_model_download() {
    if let Some(flag) = CANCEL_DOWNLOAD.get() {
        flag.store(true, Ordering::Relaxed);
    }
}

pub async fn verify_checksum(path: &PathBuf, expected: &str) -> Result<(), String> {
    let data = read(path).await.map_err(|e| e.to_string())?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let hash = format!("{:x}", hasher.finalize());

    if hash != expected {
        Err("Checksum mismatch.".into())
    } else {
        Ok(())
    }
}