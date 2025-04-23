use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalMemoryEntry {
    pub tags: Vec<String>,
    pub source: String,
    pub content: String,
    pub timestamp: String,
    pub context_link: Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct GlobalMemoryHandle(pub Arc<Mutex<Vec<GlobalMemoryEntry>>>);

impl GlobalMemoryHandle {
    pub fn new() -> Self {
        GlobalMemoryHandle(Arc::new(Mutex::new(Vec::new())))
    }

    pub fn insert(&self, entry: GlobalMemoryEntry) {
        if let Ok(mut vec) = self.0.lock() {
            vec.push(entry);
        }
    }

    pub fn search_by_tag(&self, tag: &str) -> Vec<GlobalMemoryEntry> {
        self.0.lock()
            .map(|vec| {
                vec.iter()
                    .filter(|e| e.tags.contains(&tag.to_string()))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn all(&self) -> Option<Vec<GlobalMemoryEntry>> {
        self.0.lock().ok().map(|v| v.clone())
    }
}