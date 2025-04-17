#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex, OnceLock},
    time::Duration,
};

use tauri::{AppHandle, Manager};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use portpicker::pick_unused_port;
use reqwest;
use tauri::Listener;
use tauri::Emitter;

mod config;
mod mode_manager;
mod model_selector;
mod model_downloader;
mod model_installer;
mod llama_wrapper;
mod model_registry;
mod disk;

use crate::config::*;
use model_selector::ModelChoice;
use model_registry::get_model_download_info;
use model_downloader::{
    download_model_file,
    estimate_model_file_size,
    cancel_model_download,
};
use disk::get_free_disk_space;
use model_installer::{
    install_model,
    cancel_model_install,
    check_model_ready,
    ModelStatus,
};
use mode_manager::{get_current_mode, set_current_mode};
use llama_wrapper::run_llama_inference;

struct BackendState(pub Arc<Mutex<Option<CommandChild>>>);
static ONCE_INIT: OnceLock<()> = OnceLock::new();

#[tauri::command]
fn list_project_files(project_path: String) -> Result<Vec<String>, String> {
    let root = PathBuf::from(project_path);
    let mut files = vec![];

    if root.exists() {
        for entry in walkdir::WalkDir::new(&root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            if let Ok(relative) = entry.path().strip_prefix(&root) {
                files.push(relative.to_string_lossy().to_string());
            }
        }
        Ok(files)
    } else {
        Err("Project path does not exist.".to_string())
    }
}

#[tauri::command]
fn read_file(project_path: String, relative_path: String) -> Result<String, String> {
    let full_path = PathBuf::from(&project_path).join(&relative_path);
    fs::read_to_string(&full_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_file(project_path: String, relative_path: String, content: String) -> Result<(), String> {
    let full_path = PathBuf::from(&project_path).join(&relative_path);
    fs::write(&full_path, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn open_project(project_path: String) -> Result<(), String> {
    set_last_opened_project(project_path.clone())
        .map_err(|e| format!("Failed to update config: {}", e))?;
    Ok(())
}

#[tauri::command]
fn get_recent_projects() -> Result<Vec<String>, String> {
    let config = load_config().map_err(|e| format!("Failed to load config: {}", e))?;
    Ok(config.recent_projects)
}

#[tauri::command]
fn mark_last_opened_project(path: String) -> Result<(), String> {
    set_last_opened_project(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_recommended_model() -> ModelChoice {
    model_selector::pick_optimal_model()
}

async fn wait_for_backend_ready_async(app_handle: AppHandle, port: u16) {
    println!("[tauri] Waiting for backend /health check...");
    tokio::time::sleep(Duration::from_secs(1)).await;

    let url = format!("http://127.0.0.1:{}/health", port);
    let client = reqwest::Client::new();

    for attempt in 1..=20 {
        match client.get(&url).send().await {
            Ok(resp) => {
                println!("[tauri] Attempt {}: {}", attempt, resp.status());
                if resp.status().is_success() {
                    if let Some(main) = app_handle.get_webview_window("main") {
                        let js = format!(
                            "window.__API_PORT__ = {}; window.__API_URL__ = \"http://127.0.0.1:{}\";",
                            port, port
                        );
                        let _ = main.eval(&js);
                        tokio::time::sleep(Duration::from_millis(300)).await;
                    }

                    let _ = app_handle.emit("port-ready", port);
                    println!("✓ Setup complete. System ready.");
                    let _ = app_handle.emit("sidecar-log", "✓ Setup complete. System ready.".to_string());

                    if let Some(main) = app_handle.get_webview_window("main") {
                        let _ = main.show();
                    }

                    if let Some(splash) = app_handle.get_webview_window("splashscreen") {
                        let _ = splash.close();
                    }

                    break;
                }
            }
            Err(err) => println!("[tauri] Attempt {}: Request failed: {}", attempt, err),
        }

        tokio::time::sleep(Duration::from_secs(3)).await;
    }

    let _ = app_handle.emit("sidecar-log", "⚠️ Backend did not respond.".to_string());
}

fn spawn_and_monitor_embedded_server(app_handle: AppHandle, port: u16) -> Result<(), String> {
    let cloned_handle = app_handle.clone();
    let state = app_handle.state::<BackendState>();
    let mut process_lock = state.0.lock().unwrap();

    if process_lock.is_some() {
        println!("[tauri] Sidecar already running");
        return Ok(());
    }

    let port_arg = format!("--port={}", port);
    let (mut rx, child) = app_handle
        .shell()
        .sidecar("main")
        .map_err(|e| e.to_string())?
        .args(&[port_arg])
        .spawn()
        .map_err(|e| e.to_string())?;

    *process_lock = Some(child);

    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(data) | CommandEvent::Stderr(data) = event {
                let line = String::from_utf8_lossy(&data).to_string();
                println!("[log] {}", line);
                let _ = cloned_handle.emit("sidecar-log", line);
            }
        }
    });

    Ok(())
}

#[tauri::command]
fn shutdown_sidecar(app_handle: AppHandle) -> Result<String, String> {
    let state = app_handle.state::<BackendState>();
    let mut lock = state.0.lock().unwrap();

    if let Some(mut child) = lock.take() {
        let shutdown_command = "sidecar shutdown\n";
        if let Err(e) = child.write(shutdown_command.as_bytes()) {
            println!("[tauri] Failed to write to sidecar: {}", e);
            return Err("Failed to shutdown sidecar".into());
        }
        println!("[tauri] Sidecar shutdown signal sent");
        Ok("Shutdown signal sent".into())
    } else {
        Err("No sidecar running".into())
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let _ = setup_internal_dirs();

            // 💡 First-launch logic
            let config = load_config().unwrap_or_else(|_| AppConfig {
                mode: None,
                last_opened_project: None,
                recent_projects: vec![],
                paths: AppPaths {
                    projects: "".into(),
                    uploads: "".into(),
                },
                model_file_size_estimate: None,
                cached_model_info: None,
            });

            app.manage(config.clone());

            if config.mode.is_none() {
                // First launch – show install screen
                if let Some(install_window) = app.get_webview_window("install") {
                    install_window.show().ok();
                }
                return Ok(()); // ✅ Don't continue boot sequence yet
            }

            // Safe to call state commands now
            let mode = get_current_mode().unwrap_or(None);
            if mode == Some(WinterMode::Local) {
                match check_model_ready() {
                    ModelStatus::Ready => {
                        println!("[Winter] Local model is already installed");
                    }
                    _ => {
                        println!("[Winter] Installing local model...");
                        let app_handle = app.handle().clone();
                        tauri::async_runtime::spawn(async move {
                            if let Err(err) = install_model(app_handle.clone()).await {
                                app_handle.emit("setup-progress", err).ok();
                            } else {
                                set_current_mode(WinterMode::Local).ok();
                            }
                        });
                    }
                }
            }
            let port = pick_unused_port().unwrap_or(6144);
            println!("[tauri] Spawning backend on port: {}", port);
            app.manage(BackendState(Arc::new(Mutex::new(None))));
            app.manage(port);

           // spawn_and_monitor_embedded_server(app.handle().clone(), port)?;
            Ok(())
        })
        .on_menu_event(|app, event| {
            let id = event.id().0.as_str();
            let window = app.get_webview_window("main").unwrap();
            match id {
                "new-project" => { let _ = window.emit("menu-new-project", ()); }
                "new-project-existing" => { let _ = window.emit("menu-new-project-existing", ()); }
                "new-project-git" => { let _ = window.emit("menu-new-project-git", ()); }
                "recent-projects" => {
                    let config = app.state::<AppConfig>();
                    for path in &config.recent_projects {
                        println!("Recent Project: {}", path);
                    }
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            shutdown_sidecar,
            list_project_files,
            read_file,
            write_file,
            open_project,
            get_recent_projects,
            mark_last_opened_project,
            get_recommended_model,
            get_model_download_info,
            download_model_file,
            estimate_model_file_size,
            cancel_model_download,
            cancel_model_install,
            get_current_mode,
            set_current_mode,
            get_free_disk_space,
            run_llama_inference,
        ])
        .build(tauri::generate_context!())
        .expect("error while running Tauri application")
        .run(|app, _| {
            if let Some(config) = app.try_state::<AppConfig>() {
                if let Some(last_path) = &config.last_opened_project {
                    if let Some(main) = app.get_webview_window("main") {
                        let _ = main.emit("auto-open-project", last_path.clone());
                    }
                }
            } else {
                println!("⚠️ AppConfig not available yet in `.run()`");
            }

            let port = *app.state::<u16>(); // ✅ this one is safe if you `.manage(port)` before
            let handle = app.clone();

            if ONCE_INIT.set(()).is_ok() {
                app.once_any("frontend-ready", move |_| {
                    println!("[tauri] Frontend ready. Starting backend monitor...");
                    tauri::async_runtime::spawn(wait_for_backend_ready_async(handle.clone(), port));
                });
            }
        });
}