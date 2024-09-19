use super::{method, request, Method};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt::Result as FmtResult;
use std::str::Utf8Error;
use std::str;
use super::{QueryString, QueryStringValue};
use crate::http::method::MethodError;

#[derive(Debug)]
pub struct Request<'buffer> {
    path: &'buffer str,
    query_string: Option<QueryString<'buffer>>,
    method: Method,
}

// We implement trait TryFrom<&[u8]> for Request to convert a buffer of bytes into a Request instance.
impl<'buffer> TryFrom<&'buffer[u8]> for Request<'buffer> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buffer: &'buffer[u8]) -> Result<Request<'buffer>, Self::Error> {
        // returns InvalidEncoding if the buffer is not a valid UTF-8 string
        let request = str::from_utf8(buffer)?;
        let (method,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1"{
            return Err(ParseError::InvalidProtocol)
        }

        let method: Method = method.parse()?;
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]) );
            path = &path[..i];
        }
        Ok(Self{
            path,
            method,
            query_string,
        })

    }
}

// Implement a function get_next_word that takes a request string and returns the next word in the request and the rest of the request string.
fn get_next_word(request: &str) -> Option<(&str,&str)>{
    let mut iter =  request.chars();

    for (index,c) in request.chars().enumerate(){
        if c == ' ' || c == '\r'{
            return Some((&request[..index], &request[index+1..]))
        }
    }
    None
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError{
    fn from(value: MethodError) -> Self {
        Self::InvalidMethod
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidMethod => "Invalid Method",
            Self::InvalidProtocol => "Invalid Protocol",
        }
    }
}
impl Error for ParseError {}
