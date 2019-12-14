use crate::apis::oauth::v2;

use actix_web::{HttpRequest, HttpResponse, Responder};

pub struct OAuth {
    client_authenticator: Box<dyn v2::ClientAuthenticator>,
    user_authenticator: Box<dyn v2::UserAuthenticator>,
}

impl OAuth {
    pub fn new(
        ca: Box<dyn v2::ClientAuthenticator>,
        ua: Box<dyn v2::UserAuthenticator>,
    ) -> v2::OAuth {
        return OAuth {
            client_authenticator: ca,
            user_authenticator: ua,
        };
    }

    pub fn authorize_client(&self, req: HttpRequest) -> impl Responder {
        let info = v2::UserInfo {
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
