#![allow(dead_code)]
use server::Server;
use website_handler::WebsiteHandler;

mod server;
mod https;
mod website_handler;

fn main() {
    //instantiate a server at port 8080
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler);
}




