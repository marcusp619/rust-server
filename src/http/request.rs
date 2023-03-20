use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{self, Formatter, Result};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> { 
        unimplemented!()
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.message())
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        
    }
}

impl Error for ParseError {}