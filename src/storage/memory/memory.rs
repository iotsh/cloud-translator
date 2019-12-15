use crate::runtime;
use crate::storage;

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Storage<T: runtime::Object> {
    storage: Mutex<HashMap<String, T>>,
}

impl<T: runtime::Object> Storage<T> {
    pub fn new() -> Storage<T> {
        return Storage {
            storage: Mutex::new(HashMap::new()),
        };
    }
}

impl<T: runtime::Object> storage::Storage<T> for Storage<T> {
    fn get(&mut self, key: String) -> Result<T, runtime::Error> {
        let mut storage = self.storage.lock().unwrap();
        if !storage.contains_key(&key) {
            return Err(runtime::object_not_found_error(key));
        }
        return Ok(storage.get(&key).unwrap().clone());
    }

    fn list(&mut self) -> Result<Vec<T>, runtime::Error> {
        let mut storage = self.storage.lock().unwrap();
        let mut result: Vec<T> = Vec::new();
        for (k, v) in storage.iter() {
            result.push(v.clone());
        }
        return Ok(result);
    }

    fn create(&mut self, obj: T) -> Result<T, runtime::Error> {
        let mut storage = self.storage.lock().unwrap();
        let meta = obj.get_object_meta();
        let key = meta.get_storage_key();
        if storage.contains_key(&key) {
            return Err(runtime::object_conflict_error(key));
        }
        storage.insert(key, obj.clone());
        return Ok(obj);
    }
    fn update(&mut self, obj: T) -> Result<T, runtime::Error> {
        let mut storage = self.storage.lock().unwrap();
        let meta = obj.get_object_meta();
        let key = meta.get_storage_key();
        if !storage.contains_key(&key) {
            return Err(runtime::object_not_found_error(key));
        }
        storage.insert(key, obj.clone());
        return Ok(obj);
    }
    fn delete(&mut self, key: String) -> Result<T, runtime::Error> {
        let mut storage = self.storage.lock().unwrap();
        if !storage.contains_key(&key) {
            return Err(runtime::object_not_found_error(key));
        }
        return Ok(storage.remove(&key).unwrap().clone());
    }
}
