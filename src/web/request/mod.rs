use std::{io::{BufRead, BufReader, Read, Write}, net::TcpStream};

use crate::web::response::{Header, Response, ResponseCode, Versions, WebResult};

#[derive(Debug)]
struct StatusRequest {
    method: String,
    uri: String,
    version: Versions
}

impl StatusRequest {
    fn build(line_str: String) -> WebResult<StatusRequest> {
        Result::Err(
            ResponseCode::get_400()
        )
    }

    /// Returns a unified representation of a method and uri combo.
    pub fn get_resourse_and_method(&self) -> String {
        format!("{} {}", self.method, self.uri)
    }

    pub const fn get_method(&self) -> &String {
        &self.method
    }

    pub const fn get_uri(&self) -> &String {
        &self.uri
    }

    pub const fn get_version(&self) -> &Versions {
        &self.version
    }
}

#[derive(Debug)]
pub struct RequestBackend<T: Read + Write> {
    status_line: StatusRequest,
    headers: Vec<Header>,
    body: Vec<u8>,
    response_stream: T
}

impl<T: Read + Write> RequestBackend<T> {
    pub fn build(mut stream: T) -> WebResult<RequestBackend<T>> {
        let buf_reader = BufReader::new(&mut stream);
        let mut http_lines = buf_reader.lines();

        // Build the status line: 
        // 1. Get line.
        // 2. Check for EoF.
        // 3. Check read is successful.
        // 4. Delegate to StatusRequest::build().
        let status_str = match http_lines.next() {
            None => return Result::Err(
                ResponseCode::get_400()
            ),
            Some(res) => res
        };
        let status_str = match status_str {
            Ok(s) => s,
            Err(_) => return Result::Err(
                ResponseCode::get_400()
            )
        };
        let status_line = StatusRequest::build(status_str)?;

        Result::Err(
            ResponseCode::get_400()
        )
    }

    /// Send the responnse with stored [response_socket].
    pub fn respond(&mut self, response: &Response) {
        
    }
    
    pub const fn get_method(&self) -> &String {
        &self.status_line.method
    }

    pub const fn get_uri(&self) -> &String {
        &self.status_line.uri
    }

    pub const fn get_version(&self) -> &Versions {
        &self.status_line.version
    }

    pub const fn get_headers(&self) -> &Vec<Header> {
        &self.headers
    }

    pub const fn get_body(&self) -> &Vec<u8> {
        &self.body
    }

    pub fn get_response_stream(&self) -> &T {
        &self.response_stream
    }
}

pub type Request = RequestBackend<TcpStream>;

#[cfg(test)]
mod tests;
