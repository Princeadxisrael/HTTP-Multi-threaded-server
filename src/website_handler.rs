use super::server::Handler;
use super::https::{Method,Request,Response,StatusCode};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler{
    //method to handle requests and request routing
    fn handle_request(&mut self, request:&Request)->Response {
        match request.method(){
            Method::GET=>match request.path(){
                "/"=>Response::new(StatusCode::Ok, Some("<h1> Welcome here</h1>.".to_string())),
                "/hello"=>Response::new(StatusCode::Ok, Some("<h1> Hello Hello </h1>.".to_string())),
                _ =>Response::new(StatusCode::NotFound, None)
            },
            _ =>Response::new(StatusCode::NotFound, None),
        }
    }
}