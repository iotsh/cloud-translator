use crate::runtime;

use serde::{Deserialize, Serialize};

pub trait Serializer<T> {
    fn encode(&self, obj: T) -> Result<Vec<u8>, runtime::Error>;
    fn decode(&self, data: Vec<u8>) -> Result<T, runtime::Error>;
}

pub trait Object: Clone {
    fn get_type_meta(&self) -> TypeMeta;
    fn get_object_meta(&self) -> ObjectMeta;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TypeMeta {
    pub group: String,
    pub version: String,
    pub kind: String,
}

impl TypeMeta {
    pub fn new(group: String, version: String, kind: String) -> TypeMeta {
        return TypeMeta {
            group,
            version,
            kind,
        };
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ObjectMeta {
    pub namespace: String,
    pub name: String,
}

impl ObjectMeta {
    pub fn new(namespace: String, name: String) -> ObjectMeta {
        return ObjectMeta { namespace, name };
    }
    pub fn get_storage_key(&self) -> String {
        return format!("{}/{}", self.namespace, self.name);
    }
}
