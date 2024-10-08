use std::str::FromStr;
pub enum Method{
    GET,
    PUT,
    PATCH,
    POST,
    CONNECT,
    DELETE,
    HEAD,
    OPTIONS,
    TRACE
}

//parse the method from the string using Fromstr
impl FromStr for Method{
    type Err = MethodError;
    //fn to recieve a str from request and convert into one of the method enum parameters
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "GET"=>Ok(Self::GET),
            "PUT"=>Ok(Self::GET),
            "PATCH"=>Ok(Self::PATCH),
            "POST"=>Ok(Self::POST),
            "DELETE"=>Ok(Self::DELETE),
            "HEAD"=>Ok(Self::HEAD),
            "OPTIONS"=>Ok(Self::OPTIONS),
            "TRACE"=>Ok(Self::TRACE),
            _ =>Err(MethodError)
            
        }
    }
}

pub struct MethodError;