use crate::https::{method, request, Method};
use std::{path, str};
use std::str::Utf8Error;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as fmtResult};

pub struct Request{
    path: String,
    query_string:Option<String>,
    method: Method
}

// impl Request {
//     fn from_byte_to_array(buf:&[u8])->Result<Self, String>{
//         unimplemented!()
//     }
// }

impl TryFrom<&[u8]> for Request{
    
    type Error=ParseError;

    fn try_from(buf:&[u8]) -> Result<Self, Self::Error>{
        let request=str::from_utf8(buf)?;
         let(method, request)= get_next_word(request).ok_or(ParseError::InvalidRequest)?;
         let(path, request)= get_next_word(request).ok_or(ParseError::InvalidRequest)?;
         let(protocol, _)= get_next_word(request).ok_or(ParseError::InvalidRequest)?;
    

        unimplemented!()
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

