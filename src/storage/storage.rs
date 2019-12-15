use crate::runtime;

pub trait Storage<T: runtime::Object> {
    fn get(&mut self, key: String) -> Result<T, runtime::Error>;
    fn list(&mut self) -> Result<Vec<T>, runtime::Error>;
    fn create(&mut self, obj: T) -> Result<T, runtime::Error>;
    fn update(&mut self, obj: T) -> Result<T, runtime::Error>;
    fn delete(&mut self, key: String) -> Result<T, runtime::Error>;
}
