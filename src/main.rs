pub mod apis;

use crate::apis::oauth::v2::auth;
use crate::apis::oauth::v2::routes;
use crate::apis::oauth::v2::types::fake;

use actix_web::{App, HttpServer};

fn main() {
    println!("Hello, world!");
    HttpServer::new(|| {
        let ca = Box::new(fake::ClientAuthenticator::new());
        let ua = Box::new(fake::UserAuthenticator::new());
        let oauth = auth::OAuth::new(ca, ua);
        return App::new().service(routes::routes(oauth));
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap();
}
