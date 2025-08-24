use crate::packet::PacketErr;

/// Supported HTTP versions
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum Version {
    /// Unlike HTTP versions 1.0 and 1.1, the version 0.9 is not mentioned in the first line of the packet.
    /// 
    /// For example, `GET /api` would be interpreted as HTTP/0.9.
    ///
    /// HTTP/0.9 does not have header support, but this crate does not check for that.
    /// The only indication of the version considered by this crate is in the first line of the packet.
    V0_9,
    V1_0,
    V1_1
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let version_str = match self {
            Version::V0_9 => "",
            Version::V1_0 => "HTTP/1.0",
            Version::V1_1 => "HTTP/1.1",
        };
        write!(f, "{}", version_str)
    }
}


impl Version {
    pub fn to_string(&self) -> String {
        let slice = match self {
            Version::V0_9 => "",
            Version::V1_0 => "HTTP/1.0",
            Version::V1_1 => "HTTP/1.1",
        };
        return slice.to_string();
    }

    /// Take the first line of the header and determine the HTTP version. Version 0.9 does not specify a version (e.g. `GET /some/path`).
    pub fn try_from_first_line(first_line: &str) -> Result<Self, PacketErr> {
        let mut parts: Vec<&str> = first_line.trim().split_whitespace().collect();
        parts.retain(|p| p.trim().len() != 0); // filter out empty strings if needed
               
        match parts.len() {
            2 => {
                // Version 0.9, e.g. `GET /api`
                return Ok(Self::V0_9);
            }
            3 => {} // continue
            _ => {
                // 1 or more than 3 parts -> invalid
                return Err(PacketErr::FirstLineWordCountMismatch);
            }
        }

        // we know that the length of the parts is 3 or 2
        match parts[2] {
            "HTTP/1.1" => Ok(Self::V1_1),
            "HTTP/1.0" => Ok(Self::V1_0),
            _ => Err(PacketErr::InvalidHttpVersion) // invalid
        }
    }
}


#[cfg(test)]
mod version_test {
    use super::*;

    #[test]
    fn none1() {
        assert_eq!(
            Err(PacketErr::InvalidHttpVersion),
            Version::try_from_first_line("GET /api HTTP/2.0")
        );
    }

    #[test]
    fn valid_0_9__1() {
        assert_eq!(
            Ok(Version::V0_9),
            Version::try_from_first_line("GET /api")
        );
    }

    #[test]
    fn valid_0_9__2() {
        assert_eq!(
            Ok(Version::V0_9),
            Version::try_from_first_line("POST /")
        );
    }

    #[test]
    fn valid_1_0__1() {
        assert_eq!(
            Ok(Version::V1_0),
            Version::try_from_first_line("POST / HTTP/1.0")
        );
    }

    #[test]
    fn valid_1_1__1() {
        assert_eq!(
            Ok(Version::V1_1),
            Version::try_from_first_line("POST / HTTP/1.1")
        );
    }
}
