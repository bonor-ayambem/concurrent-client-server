use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ConcurrentHashMap {
    inner: Arc<Mutex<HashMap<String, String>>>, // regular hashmap is ok
}

impl ConcurrentHashMap {

    pub fn new() -> ConcurrentHashMap {
        ConcurrentHashMap {
            inner: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn put(&self, key: String, value: String){
        let mut map = self.inner.lock().unwrap();
        map.insert(key, value);
    }

    pub fn del(&self, key: String) -> bool {
        let mut map = self.inner.lock().unwrap();
        if map.contains_key(&key) {
            map.remove(&key);
            return true;
        }
        false
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let map = self.inner.lock().unwrap();
        if map.contains_key(key) {
            return map.get(key).cloned();
        }

        None
    }
}