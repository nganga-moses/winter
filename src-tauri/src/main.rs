// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use std::{env, fs};
use std::path::PathBuf;

#[tauri::command]
fn run_python_task(task: String) -> String {
    // Figure out path to the leo-agent binary relative to the executable
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");

    let agent_path = exe_dir
        .join("resources")
        .join("src-tauri")
        .join("bundled")
        .join("winter-agent");

    let mut child = Command::new(agent_path)
        .arg(task)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn winter-agent");

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);
    let mut output = String::new();

    for line in reader.lines() {
        if let Ok(text) = line {
            output.push_str(&text);
            output.push('\n');
        }
    }

    output
}
#[tauri::command]
fn lis_project_files(project_path: String)-> Result<Vec<String>, String>{
    let root = PathBuf::from(project_path);
    let mut files = vec![];

    if root.exists(){
        for entry in walkdir::WalkDir::new(&root)
            .into_iter
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            if let Ok(relative) = entry.path().strip_prefix(&root){
                files.push(relative.to_string_lossy().to_string());
            }
        }
        Ok(files)
    }else {
        Err("Project path does not exist.".to_string())
    }
}

#[tauri::command]
fn read_file(project_path: String, relative_path: String) -> Result<String, String> {
    let full_path = PathBuf::from(&project_path).join(&relative_path);
    fs::read_to_string(&full_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_file(project_path: String, relative_path: String,content: String)-> Result<(), String>{
    let full_path = PathBuf::from(&project_path).join(&relative_path);
    fs::write(&full_path, content).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_python_task, list_project_files,read_file,write_file])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}