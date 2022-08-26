use super::method;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: method::Method,
}
