use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::fs;
use crate::agents::orchestrator::timeline::append_timeline_event;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignDecision {
    pub id: String,
    pub summary: String,
    pub made_by: String,
    pub rationale: String,
    pub timestamp: String,
}

#[derive(Debug, Default)]
pub struct ProjectMemoryInner {
    pub goal_id: Option<String>,
    pub architecture: Option<String>,
    pub decisions: Vec<DesignDecision>,
    pub file_summaries: HashMap<String, String>,
}

#[derive(Clone)]
pub struct ProjectMemoryHandle(pub Arc<Mutex<ProjectMemoryInner>>);

impl ProjectMemoryHandle {
    pub fn new() -> Self {
        ProjectMemoryHandle(Arc::new(Mutex::new(ProjectMemoryInner::default())))
    }
    pub fn get_architecture(&self) -> Option<String> {
        self.0.lock().ok().and_then(|inner| inner.architecture.clone())
    }

    pub fn set_architecture(&self, arch: &str) {
        if let Ok(mut inner) = self.0.lock() {
            inner.architecture = Some(arch.to_string());
        }
    }

    pub fn write_decision(&self, decision: DesignDecision) {
        if let Ok(mut inner) = self.0.lock() {
            inner.decisions.push(decision);
        }

    }

    pub fn update_file_summary(&self, path: &str, summary: &str) {
        if let Ok(mut inner) = self.0.lock() {
            inner.file_summaries.insert(path.to_string(), summary.to_string());
        }
    }


    pub fn all(&self) -> Option<ProjectMemoryInner> {
        self.0.lock().ok().map(|inner| inner.clone())
    }
}

