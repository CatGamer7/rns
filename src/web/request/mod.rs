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
        let parts: Vec<_> = line_str.split_whitespace().collect();

        if parts.len() < 3 {
            return Result::Err(
                ResponseCode::get_400()
            )
        }

        let version = if parts[2] == "HTTP/1.1" {
            Versions::Http1_1
        } else {
            return Result::Err(
                ResponseCode::get_400()
            )
        };

        Result::Ok(
            StatusRequest {
                method: parts[0].to_string(),
                uri: parts[1].to_string(),
                version: version
            }
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
        let mut buf_reader = BufReader::new(&mut stream);
        let mut http_lines = buf_reader.by_ref().lines();

        // Build the status line: 
        // 1. Get line.
        // 2. Check for EoF.
        // 3. Check if read is successful.
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
                ResponseCode::get_500() // Why would read fail here?
            )
        };
        let status_line = StatusRequest::build(status_str)?;

        // Read the Headers
        let mut cr_lf_consumed = false; // cr, lf marks the end of headers, even if there were none
        let mut headers = Vec::new();
        for line_res in http_lines {
            let line = match line_res {
                Ok(s) => s,
                Err(_) => return Result::Err(
                    ResponseCode::get_500() // Why would read fail here?
                )
            };

            // Must see cr, nl after all headers
            if line.is_empty() {
                cr_lf_consumed = true;
                break;
            }

            let header = Header::build(line)?;
            headers.push(header);
        }

        // Must see cr, nl after all headers
        if !cr_lf_consumed {
            return Result::Err(
                ResponseCode::get_400()
            )
        };
        
        // Collect the request body
        let mut body = Vec::new();
        if let Result::Err(_) = buf_reader.read_to_end(&mut body) {
            return Result::Err(
                ResponseCode::get_500() // Why would read fail here?
            )
        };
        
        Result::Ok(
            RequestBackend {
                status_line: status_line,
                headers: headers,
                body: body,
                response_stream: stream
            }
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
