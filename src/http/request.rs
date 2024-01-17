use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::str::Utf8Error;
use std::str;
use super::method::{Method, MethodError};


pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<String>,
    method: Method
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn query_string(&self) -> Option<&String>{
        self.query_string.as_ref() 
    }

    pub fn method(&self) -> &Method {
        &self.method
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {

        let request = str::from_utf8(buf)?;

        // ok_or is for when the option is None, ? is for the possible error we get from the get_next_word function
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol)
        }

        let method = method.parse()?;

        let mut query_string = None;
        
        if let Some(i) = path.find('?') {
            query_string = Some(path[..i].to_string());
            path = &path[i+1..];
        }

        Ok(Self {
            path,
            query_string,
            method
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {

    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..]))
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

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod"
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

// The Debug and Display will just print the same thing which I think is redundant
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError{}

