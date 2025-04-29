use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Clone)]
pub struct SessionMemory {
    sessions: Arc<Mutex<HashMap<String, String>>>, // key-value store
}

impl SessionMemory {
    pub fn new()-> Self{
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn save(&mut self,key: &str, value: &str){
        if let Ok(mut guard) = self.sessions.lock()
        {
            guard.insert(key.to_string(),value.to_string());
        }
    }
    pub fn load(&self, key:&str) -> Option<String> {
        self.sessions.lock().ok().and_then(|map| map.get(key).cloned())
    }
    pub fn all(&self)-> Option<HashMap<String, String>>{
        self.sessions.lock().ok().map(|m| m.clone())
    }
    pub fn handle(&self) -> Arc<Mutex<HashMap<String, String>>>{
        Arc::clone(&self.sessions)
    }
}
#[derive(Debug, Clone)]
pub struct SessionMemoryHandle(pub Arc<Mutex<HashMap<String, String>>>);

impl SessionMemoryHandle {
    pub fn save(&self, key: &str, value: &str) {
        if let Ok(mut guard) = self.0.lock() {
            guard.insert(key.to_string(), value.to_string());
        }
    }

    pub fn load(&self, key: &str) -> Option<String> {
        self.0.lock().ok().and_then(|map| map.get(key).cloned())
    }

    pub fn all(&self) -> Option<HashMap<String, String>> {
        self.0.lock().ok().map(|m| m.clone())
    }
}

impl From<&SessionMemory> for SessionMemoryHandle {
    fn from(mem: &SessionMemory) -> Self {
        SessionMemoryHandle(mem.handle())
    }
}