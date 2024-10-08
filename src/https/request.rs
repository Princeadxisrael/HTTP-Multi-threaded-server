use crate::https::method::{Method, MethodError};
use crate::https::request;
use std::str;
use std::str::Utf8Error;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as fmtResult};


//sample req string: GET /search?id=abc&sort=1 HTTP1.1\r\n...HEADERS...BODY
pub struct Request<'buf>{
    path: &'buf str,
    query_string:Option<&'buf str>,
    method: Method
}

// impl Request {
//     fn from_byte_to_array(buf:&[u8])->Result<Self, String>{
//         unimplemented!()
//     }
// }

//convert byte into string slice using Tryfrom and str::from_utf8
impl <'buf >TryFrom<&'buf [u8]> for Request <'buf>{
    
    type Error=ParseError;

    fn try_from(buf:&'buf [u8]) -> Result<Request<'buf>, Self::Error>{
        let request=str::from_utf8(buf)?;
         let(method, request)= get_next_word(request).ok_or(ParseError::InvalidRequest)?;
         let(mut path, request)= get_next_word(request).ok_or(ParseError::InvalidRequest)?;
         let(protocol, _)= get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1"{
            return Err(ParseError::InvalidProtocol);
        }
        let method: Method=method.parse()?; //use the parse method to convert the method string into the methdd enum
        
        let mut query_string=None;
        if let Some(i) =  path.find('?'){
            query_string=Some(&path[i+1..]);
            path=&path[..i];
        }

        Ok(Self{path, query_string, method})
    }

}

//creating a helper function to partition the request string into characters
fn get_next_word(request: &str)->Option<(&str, &str)>{
    for (i, c) in request.chars().enumerate(){
        if c == ' ' || c =='\r' {
            return Some((&request[..i], &request[i +1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self{
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError{
    fn from(_: MethodError) -> Self{
        Self::InvalidMethod
    }
}

impl ParseError{
    fn message(&self)->&str{
        match self{
            Self::InvalidRequest =>"InvalidRequest",
            Self::InvalidEncoding=>"InvalidEncoding",
            Self::InvalidProtocol=>"InvalidProtocol",
            Self::InvalidMethod=> "InvalidMethod"
        }
    }
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
                 write!(f, "{}", self.message())
             }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
                 write!(f, "{}", self.message())
             }
}


impl Error for ParseError{
}

