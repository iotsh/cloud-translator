use crate::apis::oauth::v2::auth;

use actix_web::{web, HttpRequest, Scope};
use std::rc::Rc;

// Creates routes for OAuth APIs.
pub fn routes(a: auth::OAuth) -> Scope {
    let auth = Rc::new(a);
    let scope = web::scope("/oauth/v2");
    return scope.route(
        "/authorize",
        web::get().to(move |req: HttpRequest| {
            return auth.authorize_client(req);
        }),
    );
}
