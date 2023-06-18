
use super::method::Method;

pub struct Request {
    path: String,
    query_string: Option<String>, // This indicates the field is optional
    method: Method,
}