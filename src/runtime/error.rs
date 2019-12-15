use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ErrorKind {
    NotFount,
    Conflict,
    InternalError,
    NotImplement,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

pub fn not_found_error(msg: String) -> Error {
    return Error {
        kind: ErrorKind::NotFount,
        message: msg,
    };
}

pub fn conflict_error(msg: String) -> Error {
    return Error {
        kind: ErrorKind::Conflict,
        message: msg,
    };
}

pub fn not_impelement_error(msg: String) -> Error {
    return Error {
        kind: ErrorKind::NotImplement,
        message: msg,
    };
}

pub fn object_not_found_error(key: String) -> Error {
    return not_found_error(format!("can't get object with key {}", key));
}

pub fn object_conflict_error(key: String) -> Error {
    return not_found_error(format!("conflict object with key {}", key));
}
