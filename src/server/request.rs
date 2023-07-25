use std::{str::FromStr, string::ParseError};

enum Methods {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    PATCH,
    TRACE,
}

struct Request {
    method: Methods,
    path: String,
    query_string: Option<String>,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl FromStr for Request {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}
