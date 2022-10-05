use std::str;
use std::str::Utf8Error;
use super::Method;
use super::method::MethodError;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug,Formatter, Result as FmtResult};
use super::Query;

#[derive(Debug)]
pub struct Request<'a> {
    path: &'a str,
    query_string: Option<Query<'a>>,
    method: Method,
}

impl<'a> Request<'a> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&Query> {
        self.query_string.as_ref()
    }
}

impl<'a> TryFrom<&'a [u8]> for Request<'a> {

    type Error = ParseError;

    fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {

        // match str::from_utf8(buf) {
        //     Ok(request) => { },
        //     Err(_) => Err(ParseError::INAVLID_ENCODING)     // Method - 1
        // }

        // match str::from_utf8(buf).or(Err(ParseError::INAVLID_ENCODING)){
        //     Ok(request) => { },
        //     Err(e) => Err(e)                               // Method - 2
        // }

        // let result = str::from_utf8(buf).or(Err(ParseError::INAVLID_ENCODING))?; // Method - 3

        let request = str::from_utf8(buf)?;            // Method - 4
        // match get_next_word(request) {
        //     Some((method, request)) => { },
        //     None => Err(ParseError::INVALID_REQUEST)      // Method - 1
        // }

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;  // Method - 2
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query_string = None;
        if let Some(index) = path.find("?") {
            query_string = Some(Query::from(&path[index + 1..]));
            path = &path[..index];
        }
        Ok(
            Self {
                path,
                query_string,
                method
            }
        )


    }
}

fn get_next_word(request: &str) ->  Option<(&str, &str)> {
    
    for (index, char) in request.chars().enumerate() {
        if char.is_whitespace() || char == '\r' {
            return Some((&request[..index], &request[index+1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Response",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol=> "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        ParseError::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError { }
