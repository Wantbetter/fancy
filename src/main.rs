extern crate actix;
extern crate actix_web;

use actix::Actor;
use actix_web::{http, server, App, HttpRequest};
use std::cell::Cell;

struct AppState {
    counter: Cell<usize>,
}

fn index(req: &HttpRequest<AppState>) -> String {
    let count = req.state().counter.get() + 1;
    req.state().counter.set(count);

    format!("Request number: {}", count)
}

fn main() {
    let a = 1;
}
