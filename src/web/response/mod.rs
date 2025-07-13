#[derive(Debug)]
#[derive(PartialEq)]
pub enum Versions {
    Http1_1,
    Http2,
    Http3
}

#[derive(Debug)]
pub struct Header {
    name: String,
    value: String
}

impl Header {
    pub fn build(header_str: String) -> WebResult<Header> {
        WebResult::Err(
            ResponseCode::get_400()
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

/// Shorthand for crate-specific [Result]s
type WebResult<T> = Result<T, ResponseCode>;

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
}

#[cfg(test)]
mod tests;
