use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientInfo {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub error: String,
    pub error_description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub clients: Vec<String>,
}

pub trait ClientAuthenticator {
    fn authenticate_client(&self, id: String, secret: String) -> (Option<Box<dyn Client>>, bool);
}

pub trait UserAuthenticator {
    fn authenticate_user(&self, user: String, password: String) -> (Option<Box<dyn User>>, bool);
}

pub trait Client {
    fn info(&self) -> ClientInfo;
    fn authorize(&self, code: String) -> Token;
    fn refresh(&self, refresh_token: String) -> Token;
}

pub trait User {
    fn info(&self) -> UserInfo;
}
