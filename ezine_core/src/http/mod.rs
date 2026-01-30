
mod request;
pub use request::*;

mod response;
pub use response::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE
}
