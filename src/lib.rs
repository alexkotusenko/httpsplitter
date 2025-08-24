/// Supported HTTP versions
pub enum Version {
    _0_9,
    _1_0,
    _1_1
}

impl Version {
    pub fn try_from(s: &str) -> Option<Self> {
        todo!();
    }
}


/// A header 
// TODO
pub struct Header {
    key: String,
    value: String,
}

impl Header {
    pub fn try_from(s: &str) -> Option<Self> {
        todo!();
    }
}

type StatusCodeInt = usize;

/// The status code returned with responses
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

pub struct Body(String);

pub enum Method {
    Get,
    Post, 
    // TODO rest
}

pub struct RequestPacket {
    method: Method,
    url: String,
    // TODO rest
} 

// TODO
pub struct ResponsePacket {}
