// use crate::https::method::{Method, MethodError};
use super::method::{Method,MethodError};
use std::str;
use std::str::Utf8Error;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as fmtResult};
use super::QueryString;


//sample req string: GET /search?id=abc&sort=1 HTTP1.1\r\n...HEADERS...BODY

#[derive(Debug)]
pub struct Request<'buf>{
    path: &'buf str,
    query_string:Option<QueryString<'buf>>,
    method: Method
}

//getter function for the request
impl <'buf>Request <'buf> {
    pub fn path(&self)->&str{
        &self.path
    }

    pub fn method(&self)->&Method{
        &self.method
    }

    pub fn query_string(&self)->Option<&QueryString>{
        self.query_string.as_ref() //as ref helps to return a reference to the querystring and not the option
    }
    
}

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
            query_string=Some(QueryString::from(&path[i+1..]));
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

