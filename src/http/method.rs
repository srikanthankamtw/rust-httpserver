use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
        GET,
        PUT,
        POST,
        DELETE,
        PATCH,
        CONNECT,
        OPTIONS,
        TRACE,
}

impl FromStr for Method {

        type Err = MethodError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                        GET=> Ok(Self::GET),
                        PUT=> Ok(Self::PUT),
                        POST=> Ok(Self::POST),
                        DELETE=> Ok(Self::DELETE),
                        PATCH=> Ok(Self::PATCH),
                        CONNECT=> Ok(Self::CONNECT),
                        OPTIONS=> Ok(Self::OPTIONS),
                        TRACE=> Ok(Self::TRACE),
                        _ => Err(MethodError)
                }
        }
}

pub struct MethodError;