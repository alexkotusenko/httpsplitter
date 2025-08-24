/// Taken from https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = self.to_string();
        write!(f, "{}", repr)
    }   
}
