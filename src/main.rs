#![allow(unused_variables, dead_code, non_snake_case, unreachable_patterns, unused_must_use)]

use server::Server;
use website_handler::WebsiteHandler;
use std::env;

mod server;
mod http;
mod website_handler;

fn main() {
    let public_path = env::var("PUBLIC_PATH").unwrap();
    let server = Server::new("127.0.0.1:8000".to_string());
    server.run(WebsiteHandler::new(public_path));
}
