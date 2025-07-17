use std::{fmt::{self, Display}, io::{Error, Read, Write}};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Versions {
    Http1_1,
    Http2,
    Http3
}

impl Display for Versions {
    // Http versions above 1.1 are unlikely to be implemented
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Versions::Http1_1 => write!(f, "HTTP/1.1"),
            _ => Result::Err(fmt::Error)
        }
    }
}

#[derive(Debug)]
pub struct Header {
    name: String,
    value: String
}

impl Header {
    pub fn build(header_str: String) -> WebResult<Header> {
        let parts: Vec<_> = header_str.split(":").collect();

        if parts.len() < 2 {
            return Result::Err(
                ResponseCode::get_400()
            )
        }

        Result::Ok(
            Header {
                name: parts[0].trim().to_string(),
                value: parts[1].trim().to_string()
            }
        )
    }

    pub const fn get_name(&self) -> &String {
        &self.name
    }

    pub const fn get_value(&self) -> &String {
        &self.value
    }

    pub fn to_http_str(&self) -> String {
        format!("{}: {}", self.name, self.value)
    }
}

/// Offers 2 variants for storing strings for [reason] field of [ResponseCode]
/// Static variant is used in const public API for initializing predetermined
/// responses.
/// Dynamic variant is used in generic API to allow for custom reasons also.
#[derive(Debug)]
enum ReasonStorageSpecifier {
    Static(&'static str),
    Dynamic(String)
}

/// This struct naturally serves as an error type in code and is used to
/// construct meaningful HTTP responses.
#[derive(Debug)]
pub struct ResponseCode {
    code: usize,
    reason: ReasonStorageSpecifier
}

impl ResponseCode {
    pub fn new(in_code: usize, in_reason: String) -> ResponseCode {
        ResponseCode {
            code: in_code,
            reason: ReasonStorageSpecifier::Dynamic(in_reason)
        }
    }

    pub const fn get_200() -> ResponseCode {
        ResponseCode {
            code: 200,
            reason: ReasonStorageSpecifier::Static("OK")
        }
    }

    pub const fn get_400() -> ResponseCode {
        ResponseCode {
            code: 400,
            reason: ReasonStorageSpecifier::Static("Bad Request")
        }
    }

    pub const fn get_401() -> ResponseCode {
        ResponseCode {
            code: 401,
            reason: ReasonStorageSpecifier::Static("Unauthorized")
        }
    }
    
    pub const fn get_403() -> ResponseCode {
        ResponseCode {
            code: 403,
            reason: ReasonStorageSpecifier::Static("Forbidden")
        }
    }
    
    pub const fn get_404() -> ResponseCode {
        ResponseCode {
            code: 404,
            reason: ReasonStorageSpecifier::Static("Not Found")
        }
    }
    
    pub const fn get_405() -> ResponseCode {
        ResponseCode {
            code: 405,
            reason: ReasonStorageSpecifier::Static("Method Not Allowed")
        }
    }

    pub const fn get_418() -> ResponseCode {
        ResponseCode {
            code: 418,
            reason: ReasonStorageSpecifier::Static("I'm A Teapot")
        }
    }

    pub const fn get_429() -> ResponseCode {
        ResponseCode {
            code: 429,
            reason: ReasonStorageSpecifier::Static("Too Many Requests")
        }
    }

    pub const fn get_500() -> ResponseCode {
        ResponseCode {
            code: 500,
            reason: ReasonStorageSpecifier::Static("Internal Server Error")
        }
    }
}

impl PartialEq for ResponseCode {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl Display for ResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.reason {
            ReasonStorageSpecifier::Static(st) => write!(f, "{} {}", self.code, st),
            ReasonStorageSpecifier::Dynamic(st) => write!(f, "{} {}", self.code, st)
        }
    }
}

/// Shorthand for crate-specific [Result]s
pub type WebResult<T> = Result<T, ResponseCode>;

pub struct StatusResponse {
    version: Versions,
    code: ResponseCode
}

impl StatusResponse {
    pub fn new(version: Versions, code: ResponseCode) -> StatusResponse {
        StatusResponse {
            version: version,
            code: code
        }
    }
}

impl Display for StatusResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.version, self.code)
    }
}

pub struct Response {
    status_line: StatusResponse,
    headers: Vec<Header>,
    body: Vec<u8>
}

impl Response {
    pub fn new(
        version: Versions, code: ResponseCode,
        headers: Vec<Header>, body: Vec<u8>
    ) -> Response {
        Response { 
            status_line: StatusResponse::new(version, code),
            headers: headers,
            body: body 
        }
    }

    fn get_status_line_str(&self) -> String {
        self.status_line.to_string()
    }

    fn get_headers_str(&self) -> impl Iterator<Item = String> {
        self.headers.iter().map(|header| header.to_http_str())
    }

    fn get_body(&self) -> &Vec<u8> {
        &self.body
    }

    /// Writes own contents to the provided stream.
    /// [Result] will return [Err] on any IO failure.
    pub fn respond<T: Read + Write>(&self, stream: &mut T) -> Result<(), Error> {
        // Write status
        let status = self.get_status_line_str();
        stream.write_all(status.as_bytes())?;
        stream.write_all("\r\n".as_bytes())?;

        // Write headers
        for header_str in self.get_headers_str() {
            stream.write_all(header_str.as_bytes())?;
            stream.write_all("\r\n".as_bytes())?;
        };

        // End headers
        stream.write_all("\r\n".as_bytes())?;

        // Write body
        stream.write_all(self.get_body())?;

        Result::Ok(())
    }

    /// A shorthand to send just the code in response with a IO related [Result].
    pub fn respond_code<T: Read + Write>(
        version: Versions,
        code: ResponseCode,
        stream: &mut T
    ) -> Result<(), Error> {
        let response = Response::new(version, code, Vec::new(), Vec::new());
        response.respond(stream)
    }
}

#[cfg(test)]
mod tests;
