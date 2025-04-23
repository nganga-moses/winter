use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Clone)]
pub struct TaskMemory{
    inner: Arc<Mutex<HashMap<String, String>>>, // key-value store
}

impl TaskMemory {
    pub fn new()-> Self{
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn save(&mut self,key: &str, value: &str){
        if let Ok(mut guard) = self.inner.lock()
        {
            guard.insert(key.to_string(),value.to_string());
        }
    }
    pub fn load(&self, key:&str) -> Option<String> {
        self.inner.lock().ok().and_then(|map| map.get(key).cloned())
    }
    pub fn all(&self)-> Option<HashMap<String, String>>{
        self.inner.lock().ok().map(|m| m.clone())
    }
    pub fn handle(&self) -> Arc<Mutex<HashMap<String, String>>>{
        Arc::clone(&self.inner)
    }

}
#[derive(Debug, Clone)]
pub struct TaskMemoryHandle(pub Arc<Mutex<HashMap<String, String>>>);

impl TaskMemoryHandle {
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

impl From<&TaskMemory> for TaskMemoryHandle {
    fn from(mem: &TaskMemory) -> Self {
        TaskMemoryHandle(mem.handle())
    }
}
