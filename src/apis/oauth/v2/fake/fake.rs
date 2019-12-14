use crate::apis::oauth::v2;

pub struct ClientAuthenticator {}

impl ClientAuthenticator {
    pub fn new() -> Self {
        return ClientAuthenticator {};
    }
}

impl v2::ClientAuthenticator for ClientAuthenticator {
    fn authenticate_client(
        &self,
        id: String,
        secret: String,
    ) -> (Option<Box<dyn v2::Client>>, bool) {
        return (None, false);
    }
}

pub struct UserAuthenticator {}

impl UserAuthenticator {
    pub fn new() -> Self {
        return UserAuthenticator {};
    }
}

impl v2::UserAuthenticator for UserAuthenticator {
    fn authenticate_user(
        &self,
        user: String,
        password: String,
    ) -> (Option<Box<dyn v2::User>>, bool) {
        return (None, false);
    }
}
