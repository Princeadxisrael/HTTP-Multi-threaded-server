use super::server::Handler;
use super::https::{Request,Response,StatusCode};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler{
    fn handle_request(&mut self, request:&Request)->Response {
        Response::new(StatusCode::Ok,  Some("<h1> Server Ran Succesfully</h1>".to_string()))
        
    }
}