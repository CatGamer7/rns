use std::io::{Cursor, Read, Seek, SeekFrom};

use crate::web::response::{Header, Response, ResponseCode, Versions};

#[test]
/// Test [Header] build method in normal operation.
fn build_header() {
    // "Normal" way to specify headers: one leading whitespace before value
    let header_str = "Host: www.example.com".to_string();
    let header = Header::build(header_str.clone()).unwrap();

    assert!(header.get_name() == "Host");
    assert!(header.get_value() == "www.example.com");
    assert!(header.to_http_str() == header_str);

    // Psychotic, but valid
    let header_str = "Host:www.example.com ".to_string();
    let header = Header::build(header_str.clone()).unwrap();

    assert!(header.get_name() == "Host");
    assert!(header.get_value() == "www.example.com");
    assert!(header.to_http_str() == "Host: www.example.com");
}

#[test]
/// Test [Header] build method with Bad Requests.
fn build_header_fail() {
    // No colon between name and value
    let header_str = "Host www.example.com".to_string();
    let header = Header::build(header_str.clone()).unwrap_err();

    assert!(header == ResponseCode::get_400());
}

#[test]
/// Test [Header] build method with Bad Requests.
fn response_code_equality() {
    let code_1 = ResponseCode::get_200();
    let code_2 = ResponseCode::get_400();
    let code_3 = ResponseCode::new(
        200,
        "Nonsense".to_string()
    );

    assert!(code_1 == code_3);
    assert!(code_1 != code_2);
    assert!(code_2 != code_3);
}

type MockStream = Cursor<Vec<u8>>;

#[test]
/// Test respond method.
fn respond() {
    let mut stream: MockStream = Cursor::new(
        Vec::new()
    );

    let resp = Response::new(
        Versions::Http1_1,
        ResponseCode::get_200(),
        vec![
            Header::build(
                "Content-Type: text/plain".to_string()
            ).unwrap(),
            Header::build(
                "Content-Length: 5".to_string()
            ).unwrap()
        ],
        Vec::from(
            "Hello"
        )
    );
    resp.respond(&mut stream).unwrap();

    // Test response that was written.
    let mut buf = Vec::new();
    stream.seek(SeekFrom::Start(0)).unwrap();
    stream.read_to_end(&mut buf).unwrap();
    
    let actual_resp = Vec::from(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 5\r\n\r\nHello".as_bytes()
    );

    assert!(buf == actual_resp);
}
