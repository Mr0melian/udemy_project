use std::option::Option;
use server::Server;
use http::{Method, Request};

mod server;
mod http;


fn main() {
    let server  = Server::new("127.0.0.1:8080");
    server.run();
}




