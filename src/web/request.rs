use std::{fmt::{format, Display}, net::TcpStream};

use crate::web::response::{Header, Versions, ResponseCode};

struct StatusRequest {
    method: String,
    uri: String,
    version: Versions
}

impl StatusRequest {
    fn build(line_str: String) -> Result<StatusRequest, ResponseCode> {
        Result::Err(
            ResponseCode::get_400()
        )
    }

    /// Returns a unified representation of a method and uri combo
    /// for use in route dispatching.
    pub fn get_resourse_and_method(&self) -> String {
        format!("{} {}", self.method, self.uri)
    }
}

pub struct Request {
    status_line: StatusRequest,
    headers: Vec<Header>,
    body: Vec<u8>
}

impl Request {
    pub fn build(socket: &TcpStream) -> Result<Request, ResponseCode> {
        Result::Err(
            ResponseCode::get_400()
        )
    }
}
