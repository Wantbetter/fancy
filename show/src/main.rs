extern crate actix;
extern crate actix_web;
extern crate env_logger;

use std::{env, io};

use actix::Actor;
use actix_web::{fs, http, server, App, HttpRequest, HttpResponse, Result, middleware};
use actix_web::middleware::session;
use actix_web::http::{header, Method, StatusCode};

fn index(req: &HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let sys = actix::System::new("fancy");

    let addr = server::new(
        || App::new()
            .middleware(middleware::Logger::default())
            .middleware(session::SessionStorage::new(
                session::CookieSessionBackend::signed(&[0; 32]).secure(false)
            ))
            .resource("/index", |r| r.f(index))
            .handler(
                "/",
                fs::StaticFiles::new("show/static/").unwrap().index_file("index.html")
            ))
        .bind("127.0.0.1:8080").expect("Can not bind to 127.0.0.1:8080")
        .shutdown_timeout(0)
        .start();

    println!("Starting http server: 127.0.0.1:8080");
    let _ = sys.run();
}
