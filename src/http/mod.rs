// by having the use lines here, the Method and Request types
// can be used directly from the parent module too:
// use http::method::Method will work -> regular way
// but also use http::Method -> because of this reexporting
pub use method::Method;
pub use query_string::{QueryString, Value as QueryStringValue};
pub use request::ParseError;
pub use request::Request;
pub use response::Response;
pub use status_code::StatusCode;

// pulling the submodules with the pub keyword so that they are accessible
// from outside of the parent module
pub mod method;
pub mod query_string;
pub mod request;
pub mod response;
pub mod status_code;