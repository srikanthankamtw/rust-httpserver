use std::net::TcpListener;
use crate::http::{Request, Response, ParseError};
use std::io::Read;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_badrequest(&mut self, e: &ParseError) -> Response;
}

pub struct Server {
    addr : String,
}

impl Server {

    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self, mut handler: impl Handler) {
        let v: Vec<&str> = self.addr.split(":").collect();
        println!("Server running on host {} and port {}", v[0], v[1]);

        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            let result = listener.accept();
            match result {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("{}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                },
                                Err(error) => { 
                                    handler.handle_badrequest(&error)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response {}", e);
                            }
                        },
                        Err(error) => { println!("{}", error)}
                    }
                },
                Err(error) => { println!("{:?}", error)}
            }
        }

    }
}
