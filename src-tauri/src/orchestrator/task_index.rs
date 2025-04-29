use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use serde::{Serialize, Deserialize};

/// This allows for a timeline viewer, fast search/filter by task and metadata view without opening rhe full task logs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskIndexEntry{
    pub task_id: String,
    pub agent_id: String,
    pub task_type: String,
    pub status: String,
    pub goal_id: Option<String>,
    pub timestamp: u64,
    pub revision_id: Option<u32>,
}

pub fn append_to_task_index(entry: &TaskIndexEntry)-> std::io::Result<()> {
    let index_path = dirs::home_dir()
        .expect("No home dir")
        .join("WinterData/logs/task_index.json");

    let mut entries = if index_path.exists(){
        let file = File::open(&index_path)?;
        let reader = BufReader::new(file);
        serde_json::from_reader::<_,Vec<TaskIndexEntry>>(reader).unwrap_or_default()
    } else {
        vec![]
    };
    entries.push(entry.clone());

    let  file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&index_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &entries)?;

    Ok(())
}
pub fn read_task_index()-> std::io::Result<Vec<TaskIndexEntry>>{
    let path = dirs::home_dir()
        .expect("No home dir")
        .join("WinterData/logs/task_index.json");

    if !path.exists(){
        return Ok(vec![]); // Return emty list if file doesnt exist
    }

    let content = fs::read_to_string(path)?;
    let entries: Vec<TaskIndexEntry> = serde_json::from_str(&content)?;
    Ok(entries)
}
pub fn get_failed_tasks()->std::io::Result<Vec<TaskIndexEntry>>{
    read_task_index().map(|entries|{
        entries
            .into_iter()
            .filter(|entry| entry.status=="Failed")
            .collect()
    })
}