use crate::runtime;
use crate::storage;

use rusqlite;

struct Storage<T: runtime::Object> {
    type_meta: runtime::TypeMeta,
    new_func: fn() -> T,
    connection: rusqlite::Connection,
}
