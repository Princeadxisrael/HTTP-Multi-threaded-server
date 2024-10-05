use std::io::Read;
use crate::https::Request;
use std::convert::TryFrom;
use std::net::TcpListener;
pub struct Server{
    addr: String
}

impl Server{
    //associated fn new
    pub fn new(addr:String)->Self{
        Self{
            addr
        }
    }
    //method run to run the server
    pub fn run(self){
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
                            //implement the tryfrom trait to convert the buffer from byte &[u8] to array
                            match Request::try_from(&buffer[..]) {
                               Ok(request)=>{}
                               Err(e)=>print!("failed to read connection: {}" ,e)
                            };

                        }
                    Err(e)=>println!("Failed to read from connection: {}", e)
                    }
                }
                Err(e)=>println!("An error occured:{} ", e)
            }
        }
    }
    
}