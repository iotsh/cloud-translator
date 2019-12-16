use crate::runtime;
use crate::storage;
use std::marker;

use rusqlite;

struct Storage<T: runtime::Object> {
    type_meta: runtime::TypeMeta,
    connection: rusqlite::Connection,
    _phantom: marker::PhantomData<T>,
}

impl<T: runtime::Object> Storage<T> {
    pub fn new(connection: rusqlite::Connection, obj: T) -> Storage<T> {
        return Storage {
            type_meta: obj.get_type_meta(),
            connection,
            _phantom: marker::PhantomData,
        };
    }
}

impl<T: runtime::Object> storage::Storage<T> for Storage<T> {
    fn get(&mut self, key: String) -> Result<T, runtime::Error> {
        return Err(runtime::not_impelement_error("".to_string()));
    }

    fn list(&mut self) -> Result<Vec<T>, runtime::Error> {
        return Err(runtime::not_impelement_error("".to_string()));
    }

    fn create(&mut self, obj: T) -> Result<T, runtime::Error> {
        return Err(runtime::not_impelement_error("".to_string()));
    }
    fn update(&mut self, obj: T) -> Result<T, runtime::Error> {
        return Err(runtime::not_impelement_error("".to_string()));
    }
    fn delete(&mut self, key: String) -> Result<T, runtime::Error> {
        return Err(runtime::not_impelement_error("".to_string()));
    }
}
