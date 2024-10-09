use std::io::Read;
use crate::https::{ request, ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::net::TcpListener;
pub struct Server{
    addr: String
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request)->Response;
    fn handle_bad_request(&mut self, e:&ParseError)->Response{
        println!("failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

impl Server{
    //associated fn new
    pub fn new(addr:String)->Self{
        Self{
            addr
        }
    }
    //method run to run the server
    pub fn run(self, mut handler:impl Handler){
        println!("listening on {}", self.addr);
        let listener= TcpListener::bind(&self.addr).unwrap();
        loop{
            //accept method stage incoming requests until TCP conn is established 
            match listener.accept(){
                Ok((mut stream, _))=>{
                    let mut buffer=[0;1024];
                   match stream.read(&mut buffer){
                        Ok(_)=>{
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));
                            //implement the tryfrom trait to parse the buffer from byte &[u8] to array
                       let response=match Request::try_from(&buffer[..]) {
                               Ok(request)=>handler.handle_request(&request),
                               Err(e)=>handler.handle_bad_request(&e)

                        };
                        if let Err(e) =  response.send(&mut stream){
                            println!("Failed to send response: {}", e);
                    
                        }
                       
                    }
                    Err(e)=>println!("An error occured:{} ", e),
                }
               
            }
                Err(_) => todo!(),
        }
    }
}
}