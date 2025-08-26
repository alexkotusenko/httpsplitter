use crate::packet::PacketErr;

/// The status code returned with responses
pub type StatusCodeInt = usize;

/// From <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status>
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StatusCode {
    // Informational responses
    /// 100
    Continue,
    /// 101
    SwitchingProtocols,
    /// 102
    Processing,
    /// 103
    EarlyHints,

    // Successful responses
    /// 200
    Ok,
    /// 201
    Created,
    /// 202
    Accepted,
    /// 203
    NonAuthoritativeInformation,
    /// 204
    NoContent,
    /// 205
    ResetContent,
    /// 206
    PartialContent,
    /// 207
    MultiStatus,
    /// 208
    AlreadyReported,
    /// 226
    IMUsed,

    // Redirection messages
    /// 300
    MultipleChoices,
    /// 301
    MovedPermanently,
    /// 302
    Found,
    /// 303
    SeeOther,
    /// 304
    NotModified,
    /// 305
    UseProxy,
    /// 306
    Unused,
    /// 307
    TemporaryRedirect,
    /// 308
    PermanentRedirect,

    // Client error responses
    /// 400
    BadRequest,
    /// 401
    Unauthorized,
    /// 402
    PaymentRequired,
    /// 403
    Forbidden,
    /// 404
    NotFound,
    /// 405
    MethodNotAllowed,
    /// 406
    NotAcceptable,
    /// 407
    ProxyAuthenticationRequired,
    /// 408
    RequestTimeout,
    /// 409
    Conflict,
    /// 410
    Gone,
    /// 411
    LengthRequired,
    /// 412
    PreconditionFailed,
    /// 413
    ContentTooLarge,
    /// 414
    UriTooLong,
    /// 415
    UnsupportedMediaType,
    /// 416
    RangeNotSatisfiable,
    /// 417
    ExpectationFailed,
    /// 418
    ImATeapot,
    /// 421
    MisdirectedRequest,
    /// 422
    UnprocessableContent,
    /// 423
    Locked,
    /// 424
    FailedDependency,
    /// 425
    TooEarly,
    /// 426
    UpgradeRequired,
    /// 428
    PreconditionRequired,
    /// 429
    TooManyRequests,
    /// 431
    RequestHeaderFieldsTooLarge,
    /// 451
    UnavailableForLegalReasons,


    // Server error responses
    /// 500
    InternalServerError,
    /// 501
    NotImplemented,
    /// 502
    BadGateway,
    /// 503
    ServiceUnavailable,
    /// 504
    GatewayTimeout,
    /// 505
    HttpVersionNotSupported,
    /// 506
    VariantAlsoNegotiates,
    /// 507
    InsufficientStorage,
    /// 508
    LoopDetected,
    /// 510
    NotExtended,
    /// 511
    NetworkAuthenticationRequired,
}


impl StatusCode {
    pub fn as_int(&self) -> StatusCodeInt { 
        match self {
            StatusCode::Continue => 100,
            StatusCode::SwitchingProtocols => 101,
            StatusCode::Processing => 102,
            StatusCode::EarlyHints => 103,

            StatusCode::Ok => 200,
            StatusCode::Created => 201,
            StatusCode::Accepted => 202,
            StatusCode::NonAuthoritativeInformation => 203,
            StatusCode::NoContent => 204,
            StatusCode::ResetContent => 205,
            StatusCode::PartialContent => 206,
            StatusCode::MultiStatus => 207,
            StatusCode::AlreadyReported => 208,
            StatusCode::IMUsed => 226,

            StatusCode::MultipleChoices => 300,
            StatusCode::MovedPermanently => 301,
            StatusCode::Found => 302,
            StatusCode::SeeOther => 303,
            StatusCode::NotModified => 304,
            StatusCode::UseProxy => 305,
            StatusCode::Unused => 306,
            StatusCode::TemporaryRedirect => 307,
            StatusCode::PermanentRedirect => 308,

            StatusCode::BadRequest => 400,
            StatusCode::Unauthorized => 401,
            StatusCode::PaymentRequired => 402,
            StatusCode::Forbidden => 403,
            StatusCode::NotFound => 404,
            StatusCode::MethodNotAllowed => 405,
            StatusCode::NotAcceptable => 406,
            StatusCode::ProxyAuthenticationRequired => 407,
            StatusCode::RequestTimeout => 408,
            StatusCode::Conflict => 409,
            StatusCode::Gone => 410,
            StatusCode::LengthRequired => 411,
            StatusCode::PreconditionFailed => 412,
            StatusCode::ContentTooLarge => 413,
            StatusCode::UriTooLong => 414,
            StatusCode::UnsupportedMediaType => 415,
            StatusCode::RangeNotSatisfiable => 416,
            StatusCode::ExpectationFailed => 417,
            StatusCode::ImATeapot => 418,
            StatusCode::MisdirectedRequest => 421,
            StatusCode::UnprocessableContent => 422,
            StatusCode::Locked => 423,
            StatusCode::FailedDependency => 424,
            StatusCode::TooEarly => 425,
            StatusCode::UpgradeRequired => 426,
            StatusCode::PreconditionRequired => 428,
            StatusCode::TooManyRequests => 429,
            StatusCode::RequestHeaderFieldsTooLarge => 431,
            StatusCode::UnavailableForLegalReasons => 451,

            StatusCode::InternalServerError => 500,
            StatusCode::NotImplemented => 501,
            StatusCode::BadGateway => 502,
            StatusCode::ServiceUnavailable => 503,
            StatusCode::GatewayTimeout => 504,
            StatusCode::HttpVersionNotSupported => 505,
            StatusCode::VariantAlsoNegotiates => 506,
            StatusCode::InsufficientStorage => 507,
            StatusCode::LoopDetected => 508,
            StatusCode::NotExtended => 510,
            StatusCode::NetworkAuthenticationRequired => 511,
        }
    }

    pub fn try_from_int(int: StatusCodeInt) -> Option<Self> {
        match int {
            100 => Some(Self::Continue),
            101 => Some(Self::SwitchingProtocols),
            102 => Some(Self::Processing),
            103 => Some(Self::EarlyHints),

            200 => Some(Self::Ok),
            201 => Some(Self::Created),
            202 => Some(Self::Accepted),
            203 => Some(Self::NonAuthoritativeInformation),
            204 => Some(Self::NoContent),
            205 => Some(Self::ResetContent),
            206 => Some(Self::PartialContent),
            207 => Some(Self::MultiStatus),
            208 => Some(Self::AlreadyReported),
            226 => Some(Self::IMUsed),

            300 => Some(Self::MultipleChoices),
            301 => Some(Self::MovedPermanently),
            302 => Some(Self::Found),
            303 => Some(Self::SeeOther),
            304 => Some(Self::NotModified),
            305 => Some(Self::UseProxy),
            306 => Some(Self::Unused),
            307 => Some(Self::TemporaryRedirect),
            308 => Some(Self::PermanentRedirect),

            400 => Some(Self::BadRequest),
            401 => Some(Self::Unauthorized),
            402 => Some(Self::PaymentRequired),
            403 => Some(Self::Forbidden),
            404 => Some(Self::NotFound),
            405 => Some(Self::MethodNotAllowed),
            406 => Some(Self::NotAcceptable),
            407 => Some(Self::ProxyAuthenticationRequired),
            408 => Some(Self::RequestTimeout),
            409 => Some(Self::Conflict),
            410 => Some(Self::Gone),
            411 => Some(Self::LengthRequired),
            412 => Some(Self::PreconditionFailed),
            413 => Some(Self::ContentTooLarge),
            414 => Some(Self::UriTooLong),
            415 => Some(Self::UnsupportedMediaType),
            416 => Some(Self::RangeNotSatisfiable),
            417 => Some(Self::ExpectationFailed),
            418 => Some(Self::ImATeapot),
            421 => Some(Self::MisdirectedRequest),
            422 => Some(Self::UnprocessableContent),
            423 => Some(Self::Locked),
            424 => Some(Self::FailedDependency),
            425 => Some(Self::TooEarly),
            426 => Some(Self::UpgradeRequired),
            428 => Some(Self::PreconditionRequired),
            429 => Some(Self::TooManyRequests),
            431 => Some(Self::RequestHeaderFieldsTooLarge),
            451 => Some(Self::UnavailableForLegalReasons),

            500 => Some(Self::InternalServerError),
            501 => Some(Self::NotImplemented),
            502 => Some(Self::BadGateway),
            503 => Some(Self::ServiceUnavailable),
            504 => Some(Self::GatewayTimeout),
            505 => Some(Self::HttpVersionNotSupported),
            506 => Some(Self::VariantAlsoNegotiates),
            507 => Some(Self::InsufficientStorage),
            508 => Some(Self::LoopDetected),
            510 => Some(Self::NotExtended),
            511 => Some(Self::NetworkAuthenticationRequired),

            _ => None,
        }
    }

    pub fn description(&self) -> String {
        match self {
            StatusCode::Continue => "Continue",
            StatusCode::SwitchingProtocols => "Switching Protocols",
            StatusCode::Processing => "Processing",
            StatusCode::EarlyHints => "Early Hints",

            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::Accepted => "Accepted",
            StatusCode::NonAuthoritativeInformation => "Non-Authoritative Information",
            StatusCode::NoContent => "No Content",
            StatusCode::ResetContent => "Reset Content",
            StatusCode::PartialContent => "Partial Content",
            StatusCode::MultiStatus => "Multi-Status",
            StatusCode::AlreadyReported => "Already Reported",
            StatusCode::IMUsed => "IM Used",

            StatusCode::MultipleChoices => "Multiple Choices",
            StatusCode::MovedPermanently => "Moved Permanently",
            StatusCode::Found => "Found",
            StatusCode::SeeOther => "See Other",
            StatusCode::NotModified => "Not Modified",
            StatusCode::UseProxy => "Use Proxy",
            StatusCode::Unused => "Unused",
            StatusCode::TemporaryRedirect => "Temporary Redirect",
            StatusCode::PermanentRedirect => "Permanent Redirect",

            StatusCode::BadRequest => "Bad Request",
            StatusCode::Unauthorized => "Unauthorized",
            StatusCode::PaymentRequired => "Payment Required",
            StatusCode::Forbidden => "Forbidden",
            StatusCode::NotFound => "Not Found",
            StatusCode::MethodNotAllowed => "Method Not Allowed",
            StatusCode::NotAcceptable => "Not Acceptable",
            StatusCode::ProxyAuthenticationRequired => "Proxy Authentication Required",
            StatusCode::RequestTimeout => "Request Timeout",
            StatusCode::Conflict => "Conflict",
            StatusCode::Gone => "Gone",
            StatusCode::LengthRequired => "Length Required",
            StatusCode::PreconditionFailed => "Precondition Failed",
            StatusCode::ContentTooLarge => "Content Too Large",
            StatusCode::UriTooLong => "URI Too Long",
            StatusCode::UnsupportedMediaType => "Unsupported Media Type",
            StatusCode::RangeNotSatisfiable => "Range Not Satisfiable",
            StatusCode::ExpectationFailed => "Expectation Failed",
            StatusCode::ImATeapot => "I'm a teapot",
            StatusCode::MisdirectedRequest => "Misdirected Request",
            StatusCode::UnprocessableContent => "Unprocessable Content",
            StatusCode::Locked => "Locked",
            StatusCode::FailedDependency => "Failed Dependency",
            StatusCode::TooEarly => "Too Early",
            StatusCode::UpgradeRequired => "Upgrade Required",
            StatusCode::PreconditionRequired => "Precondition Required",
            StatusCode::TooManyRequests => "Too Many Requests",
            StatusCode::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            StatusCode::UnavailableForLegalReasons => "Unavailable For Legal Reasons",

            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::NotImplemented => "Not Implemented",
            StatusCode::BadGateway => "Bad Gateway",
            StatusCode::ServiceUnavailable => "Service Unavailable",
            StatusCode::GatewayTimeout => "Gateway Timeout",
            StatusCode::HttpVersionNotSupported => "HTTP Version Not Supported",
            StatusCode::VariantAlsoNegotiates => "Variant Also Negotiates",
            StatusCode::InsufficientStorage => "Insufficient Storage",
            StatusCode::LoopDetected => "Loop Detected",
            StatusCode::NotExtended => "Not Extended",
            StatusCode::NetworkAuthenticationRequired => "Network Authentication Required",
        }
        .to_string()
    }

    pub fn code_and_description(&self) -> String {
        format!(
            "{} {}",
            self.as_int(),
            self.description()
        )
    }

    /// Try to extract the status code from the first line.
    /// Only one line expected.
    pub fn try_from_first_res_line(s: &str) -> Result<Self, PacketErr> {
        // Expected format: VERSION CODE CODE_DESC
        // E.g. `HTTP/1.0 200 OK`
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(PacketErr::InvalidStatusLine);
        }

        let status_code: StatusCodeInt = (parts[1].parse::<usize>()).map_err(|_e| PacketErr::InvalidStatusLine)?;
 
        if let Some(code_enum) = Self::try_from_int(status_code) {
            let desc = code_enum.description();

            // check if the desc matches
            if desc != parts[2] {
                return Err(PacketErr::InvalidStatusLine);
            }
            else {
                return Ok(code_enum);
            }
        }
        else {
            // The code number does not correspond to anything
            return Err(PacketErr::InvalidStatusLine);
        }

    }
}

impl std::fmt::Display for StatusCode {
    /// E.g. `200 OK` or `518 I'm a teapot`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code_and_description())
    }
}

#[cfg(test)]
mod status_code_tests {
    use super::*;

    #[test]
    fn code_desc_200() {
        assert_eq!(
            "200 OK",
            StatusCode::Ok.code_and_description().as_str()
        );
    }

    #[test]
    fn code_desc_418() {
        assert_eq!(
            "418 I'm a teapot",
            StatusCode::ImATeapot.code_and_description().as_str()
        );
    }
}
