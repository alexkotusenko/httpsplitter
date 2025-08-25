use crate::packet::PacketErr;

/// An HTTP header. 
///
/// Multi-line headers, such as the one below are rejected
/// ```text
/// Key: lorem ipsum
///     dolor sit amet
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Header {
    pub key: String,
    pub value: String,
}

impl TryFrom<&str> for Header {
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
    type Error = PacketErr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts: Option<(&str, &str)> = value.split_once(":");

        if parts.is_none() { 
            return Err(PacketErr::MalformedHeader(value.to_string()));
        }

        let parts: (&str, &str) = parts.unwrap();

        return Ok(Self {
            key: parts.0.into(),
            value: parts.1.into()
        });
    }
}

/// e.g. `Key: SomeValue`
impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
}
