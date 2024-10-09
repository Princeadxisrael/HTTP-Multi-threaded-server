use super::StatusCode;

use std::io::{Write, Result as IoResult};



#[derive(Debug)]
pub struct Response{
    status_code:StatusCode,
    body:Option<String>
}

impl Response{
    //initialize a new response
    pub fn new (status_code:StatusCode, body:Option<String>)-> Self{
        Response{status_code,body}
    }
    //send a response string to the stream with a static dispatch implementation
    pub fn send(&self, stream: &mut impl Write)->IoResult<()>{
        let body= match &self.body{
            Some(b)=>b,
            None=>""
        };
        //write to the stream
        write!(stream, "HTTP/1.1 {} {}r\n\r\n{}" , self.status_code, self.status_code.reason_phrase(), body)
    }
}