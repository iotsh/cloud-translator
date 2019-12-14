mod modules;
use modules::*;

use crate::apis::oauth::v2;
use crate::apis::oauth::v2::fake;

use actix_web::{App, HttpServer};

fn main() {
    println!("Hello, world!");
    HttpServer::new(|| {
        let ca = Box::new(fake::ClientAuthenticator::new());
        let ua = Box::new(fake::UserAuthenticator::new());
        let oauth = v2::OAuth::new(ca, ua);
        return App::new().service(v2::routes(oauth));
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap();
}
