use crate::runtime;

pub trait Storage<T: runtime::SerializedObject + runtime::TypedObject> {
    fn get(&mut self, obj: T) -> Result<T, String>;
    fn create(&mut self, obj: T) -> Result<T, String>;
    fn update(&mut self, obj: T) -> Result<T, String>;
    fn delete(&mut self, obj: T) -> Result<T, String>;
}
