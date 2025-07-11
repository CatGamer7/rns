use std::net::TcpStream;

pub enum Versions {
    Http1_1,
    Http2,
    Http3
}

pub struct Header {
    name: String,
    value: String
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
            reason: ReasonStorageSpecifier::Static("Success")
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

pub struct StatusResponse {
    version: Versions,
    code: ResponseCode
}

pub struct Response {
    status_line: StatusResponse,
    headers: Vec<Header>,
    body: Vec<u8>
}

impl Response {
    /// Consumes self and writes its contents to a TCP stream.
    pub fn respond(self, socket: &TcpStream) {
        
    }
}
