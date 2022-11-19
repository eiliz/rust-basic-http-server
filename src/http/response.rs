use super::StatusCode;
use std::io::{Write, Result as IoResult};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response{status_code, body}
    }

    /*
    Static vs dynamic dispatch in Rust
    Normally, when we call a fn the compiler will convert that into a simple call assembly instruction
    that jumps to the address of the called fn. This doesn't work when using traits/interfaces so Rust
    offers two approaches.

    To make this method work with any type that implements Write, such as TcpStream, File, etc
    we need to pass the trait as a param. But we also need to tell the compiler what way to go
    about resolving the concrete types.

    ** pub fn send(&self, stream: &mut dyn Write) -> IoResult<()> **
    One approach is dynamic dispatching which figures out the concrete types during runtime.
    Once we know what type we need for each fn call at runtime, we do a lookup inside a vtable
    which has pointers to all available implementations. This indirection has its performance cost.

    ** pub fn send(&self, stream: &mut impl Write) -> IoResult<()> **
    The second approach moves that cost to compile time by generating all possible implementations
    of a method and calling the right one for each case. This way we don't incur runtime costs but
    the compilation time will be higher and we'll also get a bigger binary.
    */
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()>{
       let body = match &self.body {
            Some(b) => b,
            None => "",
        };
       write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.reason_phrase(), body)
    }
}