// the code from requests projects, since the headers in requests is private.
// so just copy code and make it public. after requests updated, will use it directly.

extern crate hyper;

mod request;
mod response;

pub use request::Request;
pub use response::Response;
pub use response::{Codes, StatusCode};

pub type Result = hyper::Result<Response>;
pub type Error = hyper::error::Error;

