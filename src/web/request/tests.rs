use std::{io::Cursor, iter::zip};

use crate::web::{request::{RequestBackend, StatusRequest}, response::{ResponseCode, Versions}};

#[test]
/// Test [StatusRequest] build method in normal operation.
fn build_status_request() {
    let status_str = "GET /test HTTP/1.1".to_string();
    let status_req = StatusRequest::build(status_str).unwrap();
    assert!(status_req.get_resourse_and_method() == "GET /test");
    assert!(*status_req.get_version() == Versions::Http1_1);
}

#[test]
/// Test [StatusRequest] build method with Bad Requests.
fn build_status_request_fail() {
    // Wrong version
    let status_str = "GET /test HTTP/-4.1".to_string();
    let status_req = StatusRequest::build(status_str).unwrap_err();

    assert!(status_req == ResponseCode::get_400(), "must fail on invalid version");

    // Not enough data
    let status_str = "GET".to_string();
    let status_req = StatusRequest::build(status_str).unwrap_err();

    assert!(status_req == ResponseCode::get_400(), "must fail on invalid data");
    
    // Invalid format
    let status_str = "GET/testHTTP/1.1".to_string();
    let status_req = StatusRequest::build(status_str).unwrap_err();

    assert!(status_req == ResponseCode::get_400(), "must fail on invalid format");
}

type MockStream = Cursor<Vec<u8>>;

#[test]
/// Test [RequestBackend] build method in normal operation.
fn build_request() {
    let stream: MockStream = Cursor::new(
        Vec::from(
            "GET /test HTTP/1.1\r\nHost: www.example.com\r\nAccept-Language: en\r\n\r\n{\"meow\": 1}".as_bytes()
        )
    );
    let req: RequestBackend<MockStream> = RequestBackend::build(stream).unwrap();

    assert!(req.get_method() == "GET");
    assert!(req.get_uri() == "/test");
    assert!(*req.get_version() == Versions::Http1_1);
    assert!(*req.get_body() == "{\"meow\": 1}".as_bytes());

    for (result, actual) in zip(
        req.get_headers(), 
        vec!["Host: www.example.com", "Accept-Language: en"]) 
    {
        assert!(result.to_http_str() == actual);
    }
}

#[test]
/// Test [RequestBackend] build method with Bad Requests.
fn build_request_fail() {
    // Not enough data
    let stream: MockStream = Cursor::new(
        Vec::from(
            "GET /test HTTP/1.1\r\nHost: ww".as_bytes()
        )
    );
    let req = RequestBackend::build(stream).unwrap_err();
    assert!(req == ResponseCode::get_400(), "must fail on truncated request");
    
    // Invalid format
    let stream: MockStream = Cursor::new(
        Vec::from(
            "GET /test HTTP/1.1\r\nHost: www.example.com\r\nAccept-Language: en\r\n{\"meow\": 1}".as_bytes()
        )
    );
    let req = RequestBackend::build(stream).unwrap_err();
    assert!(req == ResponseCode::get_400(), "must fail on invalid format (missing cr, nl after headers)");
}
