use crate::obj::{Body, Method, Header, Version, StatusCode};

/// An error that occurs when building or parsing packets
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PacketErr {
    /// HTTP version not specified (in reponse packets) -> cannot validate required fields
    NoVersionFound, 
    /// No status code provided for reponse packets in HTTP versions that require it
    NoStatusCode,
    /// Since HTTP/0.9 packets do not have a status line and just return a body, it would be reasonable to throw this error when a HTTP/0.9 packet does not have a body. After all, the packet would be empty without it.
    NoBody,
    /// When there are not enough lines to parse the packet, or when a \r\n\r\n sequence has not been found
    InvalidLines,
    /// When there are too little or too many words in the first line
    FirstLineWordCountMismatch,
    /// When the specified HTTP method is not supported or invalid
    InvalidMethod,
    /// When the HTTP method is missing
    MissingMethod,
    /// When the URL is missing
    MissingURL,
    /// When the version is missing
    MissingVersion,
    /// When the header can't be parsed. Includes the malformed header line.
    MalformedHeader(String),
    /// When no `\r\n\r\n` sequence could be found in the packet. This is expected even if there are no headers.
    NoHeaderEndFound,
    /// When the HTTP version indicated in the packet is not supported or invalid
    InvalidHttpVersion,
    /// When the first line of a response packet (the status line) is malformed
    InvalidStatusLine,
}

/// An HTTP request packet
///
/// Example:
/// 
/// ```text
/// GET /index.html HTTP/1.1
/// Host: www.example.com
/// Accept-Encoding: gzip, deflate, br
/// Accept-Language: en-US,en;q=0.9
/// Connection: keep-alive
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestPacket {
    pub method: Method,
    /// Aka the resource
    pub url: String,
    pub version: Version,
    pub headers: Vec<Header>,
    pub body: Option<Body>,
}

impl RequestPacket {
    /// Infallibly convert get a string representation of the packet
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

impl Into<String> for RequestPacket {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Into<Vec<u8>> for RequestPacket {
    fn into(self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}


/// Transitive struct for building request packets.
///
/// Gets consumed to yield a RequestPacket
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RequestPacketBuilder {
    pub method: Option<Method>,
    pub url: Option<String>,
    pub version: Option<Version>,
    pub headers: Option<Vec<Header>>,
    pub body: Option<Body>,
}

impl RequestPacketBuilder {
    pub fn new() -> Self {
        return Self::default();
    }

    /// URL setter
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    /// Method setter
    pub fn method(mut self, method: Method) -> Self {
        self.method = Some(method);
        self
    }
    
    /// Header setter. Instantiates the header list or extends it.
    pub fn headers(mut self, headers: Vec<Header>) -> Self {
        match self.headers {
            // Extend
            Some(ref mut h) => { h.extend(headers); }
            // Instantiate
            None => { self.headers = Some(headers); }
        }   
        self
    }
        
    /// Header setter. Intantiates the list or adds a new header to it.
    pub fn header<T>(mut self, header_pair: (T, T)) -> Self  
    where T: Into<String> {
        let h = Header {
            key: header_pair.0.into(),
            value: header_pair.1.into()
        };
        match self.headers {
            Some(ref mut hdrs) => {
                hdrs.push(h);
            }
            None => {
                let v: Vec<Header> = vec![h];
                self.headers = Some(v);
            }
        }

        self
    }
    
    /// Version setter
    pub fn version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }

    /// Body setter
    pub fn body<T>(mut self, body: T) -> Self 
    where T: std::fmt::Display {
        self.body = Some(Body(format!("{body}")));
        self
    }

    // TODO content_len header function (to set the length of the packet) 
    // TODO ^ add this for the other builder as well

    /// Try to convert the builder into a request packet. Fails if the method, URL or version is missing.
    pub fn try_build(self) -> Result<RequestPacket, PacketErr> {
        // required fields
        if let None = self.method { 
            return Err(PacketErr::MissingMethod); 
        }
        if let None = self.url { 
            return Err(PacketErr::MissingURL); 
        }
        if let None = self.version { 
            return Err(PacketErr::MissingVersion);
        }
        
        Ok(RequestPacket {
            method: self.method.unwrap(),
            url: self.url.unwrap(),
            version: self.version.unwrap(),
            headers: self.headers.unwrap_or(Vec::new()),
            body: self.body,
        })
    }

    /// Try to parse packet builder from a string. Fallible.
    pub fn try_from_str(s: &str) -> Result<Self, PacketErr> {
        let mut lines: Vec<&str> = s.split("\r\n").collect::<Vec<&str>>();
        lines
            .iter_mut()
            .map(|l| l.trim())
            .collect::<Vec<&str>>()
            .retain(|l| l.len() != 0);

        if lines.len() == 0 {
            return Err(PacketErr::InvalidLines);
        }

        let first_line: &str = lines[0];
        
        // Get HTTP version
        let version: Version = Version::try_from_first_req_line(first_line)?;

        // Get method
        let fl_parts: Vec<&str> = first_line.split_whitespace()
            .map(|x| x.trim())
            .filter(|x| x.len() > 0)
            .collect::<Vec<_>>();
        if fl_parts.len() < 2 {
            // We only have one word 
            return Err(PacketErr::FirstLineWordCountMismatch);
        } else if fl_parts.len() > 3 {
            return Err(PacketErr::FirstLineWordCountMismatch);
        }

        // now we know that we have 2 or 3 words in our first line
        let method_str = fl_parts[0];
        let method_opt: Option<Method> = Method::try_from(method_str);
        if let None = method_opt {
            return Err(PacketErr::InvalidMethod);
        }
        let method = method_opt.unwrap();

        // url
        let url = fl_parts[1];

        // Headers
        // The list of lines will have a "" entry -> that is where the headers end
         
        // This occurs because we are splitting at \r\n and a\r\n\r\nb would yield "a" "" "b"
        // assert!(lines.contains(&""));
        if !lines.contains(&"") {
            return Err(PacketErr::NoHeaderEndFound);
        }

        //{
        //    let lines_len = lines.len();
        //    let second_to_last = lines_len - 1 - 1;
        //    if lines[second_to_last] != "" {
        //        // if the "" is not second to last, then tat means that there is a \r\n sequence after the body started
        //        // this is not allowed
        //        // therefore throw and err
        //        return Err(PacketErr::BodyWithABreak);
        //    }
        //}

        let mut headers: Vec<Header> = vec![];


        for (index, line) in lines.iter().enumerate() {
            if index == 0 {
                continue; // skip the first line
            }
            if *line == "" {
                break; // we are done with the header lines
            }
            
            let header_opt: Result<Header, PacketErr> = Header::try_from(*line);
            let header = header_opt?;

            headers.push(header);
        }

        // Body
        // The last "line" (where the line break is \r\n) is the body
        // NOTE: Normally, a body cannot have a \r\n sequence. But if it happens, I would like this library to be smart enough to understand that it's a part of the body
        
        // get the index of the "" (the first one) -> that is where the headers end
        let index_header_end: usize = lines
            .iter()
            .position(|x| *x == "")
            .expect("Internal Error: Could not find `\"\"` in the list of lines");
        let body_start_index = index_header_end + 1;
        // remove all the lines before this one
        // (inclusive exclusive)
        lines = lines.drain(0..body_start_index).collect();
        let body_str = lines.join("\r\n");
        let body: Option<Body> = match body_str.as_str() {
            "" => None,
            s => Some(Body(s.to_string()))
        };
        

        
        Ok(Self {
            body,
            version: Some(version),
            method: Some(method),
            url: Some(url.to_string()),
            headers: Some(headers),
        })
    }
}

#[cfg(test)]
mod request_packet_test {
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
            version: Version::try_from_first_req_line(format!("{} {} {}", method, url, version).as_str()).expect("Could not parse version"),
            body: None,
        };

        let str_repr = "GET / HTTP/1.0\r\nKey: Value\r\n\r\n";

        assert_eq!(str_repr, rp.to_string());
    }
}

/// An HTTP response packet.
///
/// **USAGE NOTE**: A HTTP/0.9 packet has no status line (which includes a version & status code) or headers, and just returns the body. This is why the `version`, `status`, and `headers` are optional.
///
/// That being said, proper value checks have been implemented, so you cannot convert a ResponsePacket into a String with `try_to_string()` when one of the required values for the specified HTTP version is lacking.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResponsePacket {
    pub version: Version,
    pub status: Option<StatusCode>,
    pub headers: Option<Vec<Header>>,
    pub body: Option<Body>,
}


impl ResponsePacket {
    /// Try to convert resposne packet into a string. Fallible because of varying requirements for different versions.
    pub fn try_to_string(&self) -> Result<String, PacketErr> {
        // Normally, if we are using a builder, if we create a ResponsePacket struct, we can be sure that it has all the required fields. But it doens't hurt to check again
        match self.version {
            Version::V0_9 => {
                // Disregards everything but the body
                // Required fields:
                // 1) Body
                if let None = self.body {
                    return Err(PacketErr::NoBody)
                }
                Ok(format!(
                    "{}", self.body.as_ref().unwrap().0
                ))
            }
            Version::V1_0 => {
                // Required fields:
                // 1) StatusCode
                if let None = self.status {
                    return Err(PacketErr::NoStatusCode);
                }
                let mut acc = String::new();
                acc.push_str(format!("{} {}\r\n", self.version, self.status.as_ref().unwrap()).as_str());
                if let Some(hdrs) = &self.headers {
                    for hdr in hdrs {
                        acc.push_str(format!("{hdr}\r\n").as_str());
                    }
                    acc.push_str("\r\n");
                } 
                if let Some(b) = self.body.as_ref() {
                    acc.push_str(b.0.as_str());
                }
                Ok(acc)
            }
            Version::V1_1 => {
                // Required fields (similar to 1.0)
                // 1) StatusCode
                if let None = self.status {
                    return Err(PacketErr::NoStatusCode);
                }
                let mut acc = String::new();
                acc.push_str(format!("{} {}\r\n", self.version, self.status.as_ref().unwrap()).as_str());
                if let Some(hdrs) = &self.headers {
                    for hdr in hdrs {
                        acc.push_str(format!("{hdr}\r\n").as_str());
                    }
                    acc.push_str("\r\n");
                } 
                if let Some(b) = self.body.as_ref() {
                    acc.push_str(b.0.as_str());
                }
                Ok(acc)
            }
        }   
    }
}

impl TryInto<String> for ResponsePacket {
    type Error = PacketErr;
    
    fn try_into(self) -> Result<String, Self::Error> {
        self.try_to_string()
    }
}

impl TryInto<Vec<u8>> for ResponsePacket {
    type Error = PacketErr;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        match self.try_to_string() {
            Ok(s) => {
                Ok(s.into_bytes())
            }
            Err(e) => Err(e)
        }
    }
}

/// Transitive struct for building response packets.
///
/// Gets consumed to yield a ResponsePacket
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct ResponsePacketBuilder {
    pub version: Option<Version>,
    pub status: Option<StatusCode>,
    pub headers: Option<Vec<Header>>,
    pub body: Option<Body>
}

impl ResponsePacketBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Status setter
    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = Some(status);
        self
    }

    /// Header setter. Instantiates the header list or extends it.
    pub fn headers(mut self, headers: Vec<Header>) -> Self {
        match self.headers {
            // Extend
            Some(ref mut h) => { h.extend(headers); }
            // Instantiate
            None => { self.headers = Some(headers); }
        }   
        self
    }

    /// Header setter. Intantiates the list or adds a new header to it.
    pub fn header<T>(mut self, header_pair: (T, T)) -> Self
    where T: Into<String> {
        let h = Header {
            key: header_pair.0.into(),
            value: header_pair.1.into()
        };
        match self.headers {
            Some(ref mut hdrs) => {
                hdrs.push(h);
            }
            None => {
                let v: Vec<Header> = vec![h];
                self.headers = Some(v);
            }
        }
        self
    }

    /// Version setter
    pub fn version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }

    /// Body setter
    pub fn body<T>(mut self, body: T) -> Self
    where T: std::fmt::Display {
        self.body = Some(Body(format!("{body}")));
        self
    }

    pub fn try_build(mut self) -> Result<ResponsePacket, PacketErr> {
        // required fields
        if let None = self.version { return Err(PacketErr::NoVersionFound) };

        let res: ResponsePacket = match self.version.unwrap() {
            Version::V0_9 => {
                // A HTTP/0.9 reponse packet consists of just the body.
                // No headers, no status line. Just the body.
                ResponsePacket {
                    version: self.version.unwrap(),
                    body: self.body,
                    status: self.status,
                    headers: self.headers,
                }
            },
            Version::V1_0 => {
                // Packet example
                // ```
                // HTTP/1.0 200 OK
                // Content-Type: text/html
                // Content-Length: 38
                // 
                // <html><body>Hello, world!</body></html>
                // ```
                if let None = self.status {
                    return Err(PacketErr::NoStatusCode);
                }
                ResponsePacket {
                    version: self.version.unwrap(),
                    status: Some(self.status.unwrap()),
                    body: self.body,
                    headers: self.headers,
                }
            },
            Version::V1_1 => {
                // Pretty much the same structure as for HTTP/1.1
                if let None = self.status {
                    return Err(PacketErr::NoStatusCode);
                }
                ResponsePacket {
                    version: self.version.unwrap(),
                    status: Some(self.status.unwrap()),
                    body: self.body,
                    headers: self.headers,
                }
            }
        };
        Ok(res)
    }

    /// Try to parse a HTTP response packet from a string.
    ///
    /// **IMPORTANT NOTE**: HTTP/0.9 packets only consist of the body, so they are pretty much unparsable. Any string is a valid HTTP/0.9 packet. Therefore, **this does NOT parse HTTP/0.9 packets**.
    ///
    /// Example of a HTTP/0.9 response pakcet:
    /// ```text
    /// <p>That's it</p>
    /// ```
    pub fn try_from_str(s: &str) -> Result<Self, PacketErr> {
        if s.trim().len() == 0 {
            return Err(PacketErr::InvalidLines);
        }

        let mut lines: Vec<&str> = s.split("\r\n").collect();
        if lines.len() == 1 || lines.len() == 2 {
            // Only one \r\n sequence found, or none at all
            // At least two are expected (After the headers
            // e.g.
            // ```
            // HTTP/1.0 200 OK\r\nHeader1: Value1\r\n\r\n
            // ```
            return Err(PacketErr::InvalidLines);
        }

        // check if the status line (the first line) starts with a supported HTTP version
        // Do not account for HTTP/0.9
        assert!(lines.len() > 0);
        let first_line = lines[0];

        // get the version
        let version_res: Result<Version, PacketErr> = Version::try_from_first_res_line(first_line);
        let version = version_res?;

        // get the status code from the first line
        let code_res: Result<StatusCode, PacketErr> = StatusCode::try_from_first_res_line(first_line);
        let code = code_res?;

        // if there is no "" in the lines list, then that means that no \r\n\r\n sequnce was found
        // this is invalid
        if !lines.contains(&"") {
            return Err(PacketErr::NoHeaderEndFound);
        }

        // parse headers
        let mut headers: Vec<Header> = vec![];
        for (index, line) in lines.iter().enumerate() {
            if index == 0 {
                continue;
            }
            if *line == "" {
                // we hit the end of the headers
                break;
            }
            match Header::try_from(*line) {
                Ok(h) => {
                    headers.push(h);
                }
                Err(e) => { return Err(e); }
            }
        }
        
        // now that we parsed the headers, parse the body
        let index_header_end: usize = lines
            .iter()
            .position(|x| *x == "")
            .expect("Internal Error: Could not find `\"\"` in the list of lines");
        let body_start_index = index_header_end + 1;
        // remove all the lines before this one
        // (inclusive exclusive)
        lines = lines.drain(0..body_start_index).collect();
        let body_str = lines.join("\r\n");
        let body: Option<Body> = match body_str.as_str() {
            "" => None,
            s => Some(Body(s.to_string()))
        };

        let collected_headers: Option<Vec<Header>> = if {headers.len()} == 0 {
            None
        } else {
            Some(headers)
        };

        Ok(Self {
            headers: collected_headers,
            version: Some(version),
            status: Some(code),
            body,
        })
    }
}

#[cfg(test)]
mod request_packet_builder_test {
    use super::*;
    #[test]
    fn too_many_words() {
        let input = "GET /api HTTP/1.0 a";
        let output = Err(PacketErr::FirstLineWordCountMismatch);
        assert_eq!(
            RequestPacketBuilder::try_from_str(input),
            output
        );
    }
}

#[cfg(test)]
mod random_body_test {
    use super::*;
    
    #[test]
    fn joined() {
        let v = vec!["a"];
        let joined = v.join("\r\n");
        assert_eq!(joined, "a");
    }
}
