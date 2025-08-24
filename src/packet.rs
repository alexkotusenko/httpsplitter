use crate::obj::{Body, Method, Header, Version};

/// An HTTP request packet
pub struct RequestPacket {
    pub method: Method,
    /// Aka the resource
    pub url: String,
    pub version: Version,
    pub headers: Vec<Header>,
    pub body: Option<Body>,
}

impl RequestPacket {
    pub fn to_string(&self) -> String {
        let mut res = String::new();

        // Start line: METHOD URL VERSION
        res.push_str(
            format!("{} {} {}\r\n", self.method.to_string(), self.url, self.version.to_string()).as_str()
        );
    
        // Headers
        for header in &self.headers {
            res.push_str(&format!("{}: {}\r\n", header.key, header.value));
        }

        // End of headers
        res.push_str("\r\n");

        // Body (if present)
        if let Some(body) = &self.body {
            res.push_str(body.0.as_str());
        }

        // No \r\n after the body

        res
    }
}


/// Transitive struct for building request packets.
///
/// Gets consumed to yield a RequestPacket
pub struct RequestPacketBuilder {
    pub method: Option<Method>,
    pub url: Option<String>,
    pub version: Option<Version>,
    pub headers: Option<Vec<Header>>,
    pub body: Option<Body>,
}

impl RequestPacketBuilder {
    pub fn url(&mut self, url: &str) {
        self.url = Some(url.to_string());
    }

    pub fn method(&mut self, method: Method) {
        self.method = Some(method);
    }

    pub fn headers(&mut self, headers: Vec<Header>) {
        self.headers = Some(headers);
    }

    pub fn version(&mut self, version: Version) {
        self.version = Some(version);
    }

    /// Consumes the request builder and tries to convert it into a baked request with the specified body.
    ///
    /// The method, version and URL are required.
    pub fn body<T>(mut self, body: T) -> Option<RequestPacket> 
    where T: std::fmt::Display {
        self.body = Some(Body(format!("{body}")));
        // required fields
        if let None = self.method { return None; }
        if let None = self.url { return None; }
        if let None = self.version { return None };

        Some(RequestPacket {
            method: self.method.unwrap(),
            url: self.url.unwrap(),
            version: self.version.unwrap(),
            headers: self.headers.unwrap_or(Vec::new()),
            body: self.body,
        })
    }


    /// Consumes the request builder and tries to convert it into a baked request packet with no body.
    ///
    /// The method, version and URL are required.
    pub fn no_body(self) -> Option<RequestPacket> {
        // required fields
        if let None = self.method { return None; }
        if let None = self.url { return None; }
        if let None = self.version { return None };
        
        Some(RequestPacket {
            method: self.method.unwrap(),
            url: self.url.unwrap(),
            version: self.version.unwrap(),
            headers: self.headers.unwrap_or(Vec::new()),
            body: None,
        })
    }
}

#[cfg(test)]
mod reponse_packet_test {
    use super::*;

    #[test]
    fn req_packet1() {
        let headers = vec![
            Header { key: "Key".into(), value: "Value".into() }
        ];
        let method = Method::Get;
        let url = "/";
        let version = "HTTP/1.0";
        
        let rp = RequestPacket {
            method: method.clone(),
            url: url.to_string(),
            headers,
            version: Version::try_from_first_line(format!("{} {} {}", method, url, version).as_str()).expect("Could not parse version"),
            body: None,
        };

        let str_repr = "GET / HTTP/1.0\r\nKey: Value\r\n\r\n";

        assert_eq!(str_repr, rp.to_string());
    }
}

// An HTTP response packet
// TODO
//pub struct ResponsePacket {}

// TODO
// Transitive struct for building response packets.
//
// Gets consumed to yield a ResponsePacket
//pub struct ResponsePacketBuilder {}
