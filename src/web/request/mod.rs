use std::{io::{BufRead, BufReader, Error, Read, Write}, net::TcpStream};

use crate::web::response::{Header, Response, ResponseCode, Versions, WebResult};

#[derive(Debug)]
struct StatusRequest {
    method: String,
    uri: String,
    version: Versions
}

impl StatusRequest {
    /// Builds the [StatusRequest] based on the string representation.
    /// Returns [ResponseCode] of 400 (HTTP 400) on fail.
    /// 
    /// # Parameters
    /// line_str - a string representation with no \r\n at the end.
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
    /// Attempt to build the request. Since the stream is consumed by the function,
    /// a simple http response will be sent to a client in case of failure
    /// (mainly HTTP 400 due to the request not adhereing to standard, although
    /// HTTP 500 is also possible if server suffers IO failure).
    /// 
    /// # Parameters
    /// stream - usually a [TcpStream] for a web server. Although, trait bounds are
    /// quite abstract to allow for all kinds of streams (useful in testing).
    /// 
    /// # Examples
    /// Example is quite bulky for a docstring so kindly refer to the unit tests.
    pub fn build(mut stream: T) -> WebResult<RequestBackend<T>> {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut http_lines = buf_reader.by_ref().lines();

        // Build the status line: 
        // 1. Get line.
        // 2. Check for EoF.
        // 3. Check if read is successful.
        // 4. Delegate to StatusRequest::build().
        let status_str = match http_lines.next() {
            None => {                
                let respond_result = Response::respond_code(
                    Versions::Http1_1, 
                    ResponseCode::get_400(), 
                    &mut stream
                );
                return match respond_result {
                    Ok(_) => Result::Err(
                        ResponseCode::get_400()
                    ),
                    Err(_) => Result::Err(
                        ResponseCode::get_500()
                    ),
                }
            },
            Some(res) => res
        };
        let status_str = match status_str {
            Ok(s) => s,
            Err(_) => { // Why would read fail here?
                let _ = Response::respond_code(
                    Versions::Http1_1, 
                    ResponseCode::get_500(), 
                    &mut stream
                );
                return Result::Err(
                    ResponseCode::get_500()
                )
            }
        };
        let status_line = StatusRequest::build(status_str)?;

        // Read the Headers
        let mut cr_lf_consumed = false; // cr, lf marks the end of headers, even if there were none
        let mut headers = Vec::new();
        for line_res in http_lines {
            let line = match line_res {
                Ok(s) => s,
                Err(_) => { // Why would read fail here?
                    let _ = Response::respond_code(
                        Versions::Http1_1, 
                        ResponseCode::get_500(), 
                        &mut stream
                    );
                    return Result::Err(
                        ResponseCode::get_500()
                    )
                }
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
            let respond_result = Response::respond_code(
                Versions::Http1_1, 
                ResponseCode::get_400(), 
                &mut stream
            );
            return match respond_result {
                Ok(_) => Result::Err(
                    ResponseCode::get_400()
                ),
                Err(_) => Result::Err(
                    ResponseCode::get_500()
                ),
            }
        };
        
        // Collect the request body
        let mut body = Vec::new();
        if let Result::Err(_) = buf_reader.read_to_end(&mut body) { // Why would read fail here?
            let _ = Response::respond_code(
                Versions::Http1_1, 
                ResponseCode::get_500(), 
                &mut stream
            );
            return Result::Err(
                ResponseCode::get_500()
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

    /// Send the response with the stored [response_stream].
    pub fn respond(&mut self, response: &Response) -> Result<(), Error> {
        response.respond(&mut self.response_stream)
    }
    
    pub const fn get_method(&self) -> &String {
        &self.status_line.get_method()
    }

    pub const fn get_uri(&self) -> &String {
        &self.status_line.get_uri()
    }

    pub const fn get_version(&self) -> &Versions {
        &self.status_line.get_version()
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

/// A most useful shorthand for HTTP server
pub type Request = RequestBackend<TcpStream>;

#[cfg(test)]
mod tests;
