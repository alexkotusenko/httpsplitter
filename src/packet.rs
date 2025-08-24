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
    /// When there are not enough lines to parse the destination or method (so basically none at all)
    NotEnoughLines,
    /// When there are too little or too many words in the first line
    FirstLineWordCountMismatch,
    /// When the specified HTTP method is not supported or invalid
    InvalidMethod,
    /// When the header can't be parsed. Includes the malformed header line.
    MalformedHeader(String),
    /// When no `\r\n\r\n` sequence could be found in the packet. This is expected even if there are no headers.
    NoHeaderEndFound,
    /// When the HTTP version indicated in the packet is not supported or invalid
    InvalidHttpVersion,
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
#[derive(Clone, Debug, Eq, PartialEq)]
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

    pub fn try_from_str(s: &str) -> Result<Self, PacketErr> {
        let mut lines: Vec<&str> = s.split("\r\n").collect::<Vec<&str>>();
        lines
            .iter_mut()
            .map(|l| l.trim())
            .collect::<Vec<&str>>()
            .retain(|l| l.len() != 0);

        if lines.len() == 0 {
            return Err(PacketErr::NotEnoughLines);
        }

        let first_line: &str = lines[0];
        
        // Get HTTP version
        let version: Version = Version::try_from_first_line(first_line)?;

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
            
            let header_opt: Option<Header> = Header::try_from(line);
            if let None = header_opt {
                return Err(PacketErr::MalformedHeader(line.to_string()));
            }

            headers.push(header_opt.unwrap());
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
        let body_str = lines.join("\n\r");
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
            version: Version::try_from_first_line(format!("{} {} {}", method, url, version).as_str()).expect("Could not parse version"),
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
    pub fn try_to_string(&self) -> Result<String, PacketErr> {
        // Normally, if we are using a builder, if we create a ResponsePacket struct, we can be sure that it has all the required fields. But it doens't hurt to check again
        match self.version {
            Version::V0_9 => {
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

    pub fn status(&mut self, status: StatusCode) {
        self.status = Some(status);
    }

    pub fn headers(&mut self, headers: Vec<Header>) {
        self.headers = Some(headers);
    }

    pub fn version(&mut self, version: Version) {
        self.version = Some(version);
    }

    pub fn body<T>(self, body: T) -> Result<ResponsePacket, PacketErr> 
    where T: std::fmt::Display {
        return self.opt_body(Some(body));
    }

    pub fn no_body(self) -> Result<ResponsePacket, PacketErr> {
        return self.opt_body::<String>(None); // Pass `String` for trait bounds (<T> must be formattable)
    }

    fn opt_body<T>(mut self, body: Option<T>) -> Result<ResponsePacket, PacketErr> 
    where T: std::fmt::Display {
        
        let parsed_body: Option<Body> = {
            if let Some(b) = body {
                Some(Body(format!("{b}")))
            }
            else {
                None
            }
        };
        self.body = parsed_body;
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
