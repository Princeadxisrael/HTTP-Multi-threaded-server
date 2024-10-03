use server::Server;
use https::Method;
use https::Request;

mod server;
mod https;

fn main() {
    //instantiate a server at port 8080
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}




