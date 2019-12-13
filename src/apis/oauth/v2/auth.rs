use crate::apis::oauth::v2::types;

use actix_web::{HttpRequest, HttpResponse, Responder};

pub struct OAuth {
    client_authenticator: Box<dyn types::ClientAuthenticator>,
    user_authenticator: Box<dyn types::UserAuthenticator>,
}

impl OAuth {
    pub fn new(
        ca: Box<dyn types::ClientAuthenticator>,
        ua: Box<dyn types::UserAuthenticator>,
    ) -> OAuth {
        return OAuth {
            client_authenticator: ca,
            user_authenticator: ua,
        };
    }

    pub fn authorize_client(&self, req: HttpRequest) -> impl Responder {
        let info = types::UserInfo {
            id: "123".to_string(),
            name: "ali".to_string(),
            clients: vec!["kkk".to_string(), "222".to_string()],
        };

        let result = serde_json::to_string(&info).unwrap();
        return HttpResponse::Ok()
            .content_type("application/json")
            .body(result);
    }
}
