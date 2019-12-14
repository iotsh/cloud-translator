pub trait SerializedObject {
    fn encode(&self) -> Result<Vec<u8>, String>;
    fn decode(&mut self, data: Vec<u8>) -> Result<(), String>;
}

pub trait TypedObject {
    fn group(&self) -> String;
    fn version(&self) -> String;
    fn kind(&self) -> String;
}
