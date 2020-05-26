//! # HTTP server capability data structures
//!
//! This module contains data types for the `wascc:http_server` and `wascc:httpclient` capabilities

use crate::Sample;
use serde::ser::Serialize;
use std::collections::HashMap;

/// Operation invoked on a host to perform an HTTP request
pub const OP_PERFORM_REQUEST: &str = "PerformRequest";
/// Operation invoked on an actor in response to an inbound HTTP request
pub const OP_HANDLE_REQUEST: &str = "HandleRequest";

/// Describes an HTTP request
#[derive(Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// The HTTP method (e.g. GET, PUT, DELETE)
    pub method: String,
    /// The path or URL of the request, leading slashes may not be trimmed
    pub path: String,
    /// The query string portion of the URL
    pub query_string: String,
    /// The request headers as a map of key-value pairs
    #[serde(default)]
    pub header: HashMap<String, String>,
    /// The raw bytes of the request body
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub body: Vec<u8>,
}

impl Sample for Request {
    fn sample() -> Self {
        Request {
            method: "GET".to_string(),
            path: "/foo".to_string(),
            query_string: "a=1&b=2".to_string(),
            header: sample_header(),
            body: b"This is the body of a request".to_vec(),
        }
    }
}

fn sample_header() -> HashMap<String, String> {
    let mut hm = HashMap::new();
    hm.insert("accept".to_string(), "application/json".to_string());
    hm.insert("dummy".to_string(), "value".to_string());

    hm
}

/// Represents an HTTP response
#[derive(Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// The response's numerical status code (e.g. 200)
    pub status_code: u32,
    /// The string version of the status (e.g. 'OK')
    pub status: String,
    #[serde(default)]
    /// HTTP response headers as key-value pairs.
    pub header: HashMap<String, String>,
    /// The raw bytes of the body
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub body: Vec<u8>,
}

impl Sample for Response {
    fn sample() -> Self {
        Response {
            status_code: 200,
            status: "OK".to_string(),
            header: sample_header(),
            body: b"This is the body of a response".to_vec(),
        }
    }
}

impl Response {
    /// Creates a response with a given status code and serializes the given payload as JSON
    pub fn json<T>(payload: T, status_code: u32, status: &str) -> Response
    where
        T: Serialize,
    {
        Response {
            body: serde_json::to_string(&payload).unwrap().into_bytes(),
            header: HashMap::new(),
            status: status.to_string(),
            status_code,
        }
    }

    /// Handy shortcut for creating a 404/Not Found response
    pub fn not_found() -> Response {
        Response {
            status: "Not Found".to_string(),
            status_code: 404,
            ..Default::default()
        }
    }

    /// Useful shortcut for creating a 200/OK response
    pub fn ok() -> Response {
        Response {
            status: "OK".to_string(),
            status_code: 200,
            ..Default::default()
        }
    }

    /// Useful shortcut for creating a 500/Internal Server Error response
    pub fn internal_server_error(msg: &str) -> Response {
        Response {
            status: "Internal Server Error".to_string(),
            status_code: 500,
            body: msg.to_string().as_bytes().into(),
            ..Default::default()
        }
    }

    /// Shortcut for creating a 400/Bad Request response
    pub fn bad_request() -> Response {
        Response {
            status: "Bad Request".to_string(),
            status_code: 400,
            ..Default::default()
        }
    }
}
