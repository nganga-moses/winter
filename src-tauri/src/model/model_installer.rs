use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        OnceLock,
    },
};

use sha2::{Digest, Sha256};
use tokio::{fs::{self, read, OpenOptions}, io::AsyncWriteExt};
use tauri::{AppHandle};
use tauri::Emitter;
use futures_util::stream::StreamExt;

#[derive(Debug, Clone, PartialEq)]
pub enum ModelStatus {
    NotInstalled,
    Downloading,
    Ready,
}

static CANCEL_FLAG: OnceLock<AtomicBool> = OnceLock::new();

const MODEL_NAME: &str = "mistral";
const MODEL_QUANT: &str = "Q4_K_M";
const MODEL_URL: &str = "https://your.cdn.com/mistral/Q4_K_M.gguf";
const MODEL_SHA256: &str = "expected_sha256";

fn model_path() -> PathBuf {
    dirs::home_dir()
        .unwrap()
        .join("WinterData/models")
        .join(MODEL_NAME)
        .join(format!("{}.gguf", MODEL_QUANT))
}

fn tmp_path() -> PathBuf {
    model_path().with_file_name(format!("{}.partial", MODEL_QUANT))
}

pub fn check_model_ready() -> ModelStatus {
    let path = model_path();
    if path.exists() {
        ModelStatus::Ready
    } else if tmp_path().exists() {
        ModelStatus::Downloading
    } else {
        ModelStatus::NotInstalled
    }
}

#[tauri::command]
pub async fn install_model(app: AppHandle) -> Result<String, String> {
    let path = model_path();
    let tmp = tmp_path();

    fs::create_dir_all(path.parent().unwrap())
        .await
        .map_err(|e| format!("Directory error: {}", e))?;

    let resume_from = if tmp.exists() {
        tmp.metadata().ok().map(|m| m.len()).unwrap_or(0)
    } else {
        0
    };

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&tmp)
        .await
        .map_err(|e| format!("File error: {}", e))?;

    let client = reqwest::Client::new();
    let mut request = client.get(MODEL_URL);

    if resume_from > 0 {
        request = request.header("Range", format!("bytes={}-", resume_from));
    }

    let resp = request
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;

    let total_size = resp
        .headers()
        .get("content-length")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .map(|len| len + resume_from)
        .unwrap_or(0);

    let cancel = CANCEL_FLAG.get_or_init(|| AtomicBool::new(false));
    cancel.store(false, Ordering::Relaxed);

    let mut stream = resp.bytes_stream();
    let mut downloaded = resume_from;

    app.emit("setup-progress", "⬇️ Downloading model...").ok();

    while let Some(chunk) = stream.next().await {
        if cancel.load(Ordering::Relaxed) {
            return Err("❌ Download cancelled.".into());
        }

        let chunk = chunk.map_err(|e| format!("Stream error: {}", e))?;
        file.write_all(&chunk)
            .await
            .map_err(|e| format!("Write error: {}", e))?;
        downloaded += chunk.len() as u64;

        let percent = (downloaded as f64 / total_size as f64 * 100.0) as u8;
        app.emit("model-download-progress", percent).ok();
    }

    file.flush().await.map_err(|e| e.to_string())?;
    drop(file);

    fs::rename(&tmp, &path)
        .await
        .map_err(|e| format!("Rename error: {}", e))?;

    verify_checksum(&path).await?;

    app.emit("setup-progress", "✅ Model ready.").ok();

    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn cancel_model_install() {
    if let Some(flag) = CANCEL_FLAG.get() {
        flag.store(true, Ordering::Relaxed);
    }
}

async fn verify_checksum(path: &PathBuf) -> Result<(), String> {
    let data = read(path).await.map_err(|e| e.to_string())?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let hash = format!("{:x}", hasher.finalize());

    if hash != MODEL_SHA256 {
        Err("❌ Checksum mismatch!".into())
    } else {
        Ok(())
    }
}