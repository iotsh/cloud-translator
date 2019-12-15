use crate::runtime;
use crate::runtime::Object;
use crate::storage;
use crate::storage::memory;
use crate::storage::Storage;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TestObject {
    object_meta: runtime::ObjectMeta,
    spec: TestObjectSpec,
    status: TestObjectStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TestObjectSpec {
    test_field: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TestObjectStatus {
    reason: String,
}

impl runtime::Object for TestObject {
    fn get_type_meta(&self) -> runtime::TypeMeta {
        return runtime::TypeMeta {
            group: "tests".to_string(),
            version: "v1".to_string(),
            kind: "TestObject".to_string(),
        };
    }
    fn get_object_meta(&self) -> runtime::ObjectMeta {
        return self.object_meta.clone();
    }
}

#[test]
fn test_storage() {
    let mut storage: memory::Storage<TestObject> = memory::Storage::new();
    let obj = TestObject {
        object_meta: runtime::ObjectMeta::new(
            "test-namespace".to_string(),
            "test-name".to_string(),
        ),
        spec: TestObjectSpec { test_field: 122 },
        status: TestObjectStatus {
            reason: "none".to_string(),
        },
    };
    storage.create(obj.clone());
    let result = storage.list().unwrap();
    assert_eq!(result.len(), 1);
    let x = storage
        .get(obj.get_object_meta().get_storage_key())
        .unwrap();
    assert_eq!(x.spec.test_field, 122);
    let x = storage
        .get(obj.get_object_meta().get_storage_key())
        .unwrap();
    assert_eq!(x.status.reason, "none");
    println!("{:?}", x)
}
