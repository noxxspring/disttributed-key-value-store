use std::collections::HashMap;
use std::sync::{Arc, Mutex};


/// A struct that wraps the shared key-value store.
#[derive(Clone)]
pub struct Node {
    store: Arc<Mutex<HashMap<String, String>>>,
}

impl Node {
    /// create a new Node with an empty key-value store
    pub fn new() -> Self {
        Node {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// set a skey valu pair in the store
    pub fn set (&self, key: String, value: String) {
        let mut store = self.store.lock().unwrap();
        store.insert(key, value);
    }

    ///Get a value for a given key
    pub fn get(&self, key: String) -> Option<String> {
        let store = self.store.lock().unwrap();
        store.get(&key).cloned()
    }

    ///Delete  a key value pair from the store 
    pub fn delete(&self, key: String) -> bool {
        let mut store = self.store.lock().unwrap();
        store.remove(&key).is_some()
        
    }
}