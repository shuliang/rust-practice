mod links;
mod route_handlers;
mod state;

use crate::route_handlers::{add_link, index, links, rm_link};
use crate::state::State;
use actix_web::middleware::Logger;
use actix_web::{http, server, App};
use log::info;
use std::env;

fn init_env() {
    env::set_var("RUST_LOG", "linksnap=info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    info!("Starting http server: 127.0.0.1:8080");
}

fn main() {
    init_env();
    let system = actix::System::new("linksnap");
    let state = State::init();

    let web_app = move || {
        App::with_state(state.clone())
            .middleware(Logger::default())
            .route("/", http::Method::GET, index)
            .route("/links", http::Method::GET, links)
            .route("/add", http::Method::POST, add_link)
            .route("/rm", http::Method::DELETE, rm_link)
    };

    server::new(web_app).bind("127.0.0.1:8080").unwrap().start();
    let _ = system.run();
}
