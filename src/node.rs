use std::collections::HashMap;


/// A struct that wraps the shared key-value store.
#[derive(Clone)]
pub struct KVStore {
    data: HashMap<String, String>, // Store the key-value pairs
}

impl KVStore {
    /// Creates a new empty KVStore.
    pub fn new() -> Self {
        KVStore {
            data: HashMap::new(),
        }
    }


    /// Set a key-value pair in the store.
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Get the value for a given key.
    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    /// Delete a key-value pair from the store.
    pub fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

     /// Get a list of all keys stored in the KVStore.
     pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn update(&mut self, key: String, value: String) -> Option<String> {
        if self.data.contains_key(&key) {
            let old_value = self.data.insert(key, value);
            Some(old_value.unwrap()) // Return the old value before it was updated
        } else {
            None // Return None if the key doesn't exist
        }
    }
}
