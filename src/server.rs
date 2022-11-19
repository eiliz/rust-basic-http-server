// this includes the definition of the trait; has to be included even if the
// actual implementation exists on the stream
use std::io::Read;
use std::net::TcpListener;
// super means go one level up to the parent === ../
// crate means go as many levels up as needed until you get to the root
use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);
        // makes the sys calls to create a socket bound to the address it's passed
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            // starts the actual listening for incomming connections
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // inits an array of size 1024 with all elements set to 0
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            // when we implemented try_from for the Request struct we used &[u8] as the type param
                            // but buffer is of [0; 1024] type => they don't match so we either cast it with
                            // &buffer as &[u8] or we just create a slice over the entire array with &buffer[..]
                            // Request::try_from(&buffer as &[u8]);
                            // Request::try_from(&buffer[..]);
                            // let res: &Result<Request, _> = &buffer[..].try_into();

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(req) => {
                                    dbg!(&req);
                                    handler.handle_request(&req)
                                }
                                Err(e) => {
                                    println!("Failed to pass the request: {}", e);
                                    handler.handle_bad_request(&e)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed reading from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
