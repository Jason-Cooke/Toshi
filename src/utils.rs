use futures::Future;
use http::{Response, StatusCode};
use http::header::CONTENT_TYPE;
use hyper::Body;
use serde::Serialize;

use crate::error::Error;
use crate::results::ErrorResponse;

pub fn with_body<T>(body: T) -> http::Response<Body>
    where
        T: Serialize,
{
    let json = serde_json::to_vec::<T>(&body).unwrap();

    Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(json))
        .unwrap()
}

pub fn error_response(code: StatusCode, e: Error) -> http::Response<Body> {
    let mut resp = with_body(ErrorResponse::from(e));
    *resp.status_mut() = code;
    resp
}

pub fn empty_with_code(code: StatusCode) -> http::Response<Body> {
    Response::builder().status(code).body(Body::empty()).unwrap()
}

pub fn not_found() -> impl Future<Output=Result<Response<Body>, hyper::Error>> {
    async {
        let not_found = empty_with_code(StatusCode::NOT_FOUND);
        Ok(not_found)
    }
}

pub struct Paths(pub Option<String>, pub Option<String>, pub Option<String>);

pub fn parse_path(path: &str) -> Paths {
    let eles: Vec<&str> = path.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();
    if eles.is_empty() {
        Paths(None, None, None)
    } else if eles.len() == 1 {
        Paths(Some(eles[0].to_string()), None, None)
    } else if eles.len() == 2 {
        Paths(Some(eles[0].to_string()), Some(eles[1].to_string()), None)
    } else {
        Paths(Some(eles[0].to_string()), Some(eles[1].to_string()), Some(eles[0].to_string()))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_parse_path() {
        let root = "/";
        let one = "/path";
        let two = "/path/two";

        let parsed_root = parse_path(root);
        let parsed_one = parse_path(one);
        let parsed_two = parse_path(two);
//        assert_eq!(parsed_root.len(), 0);
//        assert_eq!(parsed_one, 1);
        assert_eq!(parsed_one.0.unwrap(), "path");
//        assert_eq!(parsed_two.len(), 2);
        assert_eq!(parsed_two.0.unwrap(), "path");
        assert_eq!(parsed_two.1.unwrap(), "two");
    }
}
