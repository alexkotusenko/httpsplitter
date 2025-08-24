/// Supported HTTP versions
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Version {
    /// Unlike HTTP versions 1.0 and 1.1, the version 0.9 is not mentioned in the first line of the packet.
    /// 
    /// For example, `GET /api` would be interpreted as HTTP/0.9
    v0_9,
    v1_0,
    v1_1
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let version_str = match self {
            Version::v0_9 => "",
            Version::v1_0 => "HTTP/1.0",
            Version::v1_1 => "HTTP/1.1",
        };
        write!(f, "{}", version_str)
    }
}

impl Version {
    pub fn to_string(&self) -> String {
        let slice = match self {
            Version::v0_9 => "",
            Version::v1_0 => "HTTP/1.0",
            Version::v1_1 => "HTTP/1.1",
        };
        return slice.to_string();
    }

    /// Take the first line of the header and determine the HTTP version. Version 0.9 does not specify a version (e.g. `GET /some/path`).
    pub fn try_from_first_line(first_line: &str) -> Option<Self> {
        let mut parts: Vec<&str> = first_line.trim().split_whitespace().collect();
        parts.retain(|p| p.trim().len() != 0); // filter out empty strings if needed
               
        match parts.len() {
            2 => {
                // Version 0.9, e.g. `GET /api`
                return Some(Self::v0_9);
            }
            3 => {} // continue
            _ => {
                return None; // 1 or more than 3 parts -> invalid
            }
        }

        // we know that the length of the parts is 3
        assert_eq!(parts.len(), 3);
        match parts[2] {
            "HTTP/1.1" => Some(Self::v1_1),
            "HTTP/1.0" => Some(Self::v1_0),
            _ => None // invalid
        }
    }
}

#[cfg(test)]
mod version_test {
    use super::*;

    #[test]
    fn none1() {
        assert_eq!(
            None,
            Version::try_from("GET /api HTTP/2.0")
        );
    }

    #[test]
    fn valid_0_9__1() {
        assert_eq!(
            Some(Version::v0_9),
            Version::try_from("GET /api")
        );
    }

    #[test]
    fn valid_0_9__2() {
        assert_eq!(
            Some(Version::v0_9),
            Version::try_from("POST /")
        );
    }

    #[test]
    fn valid_1_0__1() {
        assert_eq!(
            Some(Version::v1_0),
            Version::try_from("POST / HTTP/1.0")
        );
    }

    #[test]
    fn valid_1_1__1() {
        assert_eq!(
            Some(Version::v1_1),
            Version::try_from("POST / HTTP/1.1")
        );
    }
}

/// An HTTP header. 
///
/// Multi-line headers, such as the one below are rejected
/// ```text
/// Key: lorem ipsum
///     dolor sit amet
/// ```
pub struct Header {
    pub key: String,
    pub value: String,
}

impl Header {
    /// Assume the following header format:
    /// ```text
    /// Key: Value
    /// ```
    /// or
    /// ```text
    /// Multi-Word-Key: several, values, on, the, same, line
    /// ```
    /// Wrapped headers are rejected, so this would not be allowed
    /// ```text
    /// Multi-Line-Key: lorem
    ///     ipsum
    ///         dolor
    /// ```
    pub fn try_from(s: &str) -> Option<Self> {
        let mut parts: Option<(&str, &str)> = s.split_once(":");

        if parts.is_none() { return None };

        let parts = parts.unwrap();
        return Some(Self {
            key: parts.0.into(),
            value: parts.1.into()
        });
    }
}



/// The status code returned with responses
pub type StatusCodeInt = usize;

pub enum StatusCode {
    Ok, // or what is it called?
    // Etc
}

impl StatusCode {
    pub fn as_int(&self) -> StatusCodeInt { 
        todo!();
    }

    pub fn try_from_int(&self) -> Option<Self> {
        todo!();
    }
}

pub struct Body(pub String);

impl Body {
    pub fn is_valid_json(&self) -> bool {
        serde_json::from_str::<serde_json::Value>(self.0.as_str()).is_ok()
    }
}

/// Taken from https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods
pub enum Method {
    Get,
    Head,
    Options,
    Trace,
    Put,
    Delete,
    Post,
    Patch,
    Connect,
}

impl Method {
    pub fn try_from(s: &str) -> Option<Self> {
        match s.trim() {
            "GET" => Some(Self::Get),
            "HEAD" => Some(Self::Head),
            "OPTIONS" => Some(Self::Options),
            "TRACE" => Some(Self::Trace),
            "PUT" => Some(Self::Put),
            "DELETE" => Some(Self::Delete),
            "POST" => Some(Self::Post),
            "PATCH" => Some(Self::Patch),
            "CONNECT" => Some(Self::Connect),
            _ => None
        }
    }

    pub fn to_string(&self) -> String {
        let slice = match &self {
            Self::Get => "GET",
            Self::Head => "HEAD",
            Self::Options => "OPTIONS",
            Self::Trace => "TRACE",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Post => "POST",
            Self::Patch => "PATCH",
            Self::Connect => "CONNECT"
        };
        slice.to_string()
    }
}
