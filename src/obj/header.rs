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
