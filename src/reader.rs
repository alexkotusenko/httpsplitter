// reader.rs
// optional feature

/// Read from buffer until `\r\n`. `\r\n` is included at the end if found, and excluded if buffer end reached.
pub fn read_until_crlf<R: std::io::Read>(reader: &mut R) -> std::io::Result<Vec<u8>> {
    // Buffer to store the bytes read from the input
    let mut buffer = Vec::new();

    // Temporary buffer to read one byte at a time
    let mut temp = [0u8; 1];

    // Read loop: continue reading until we find the CRLF sequence (\r\n)
    while reader.read(&mut temp)? == 1 {
        // Append the byte to our main buffer
        buffer.push(temp[0]);

        // Check if the last two bytes are \r\n
        let len = buffer.len();
        if len >= 2 && buffer[len - 2] == b'\r' && buffer[len - 1] == b'\n' {
            break; // Stop reading once CRLF is found
        }
    }

    // Return the collected bytes, including the CRLF
    Ok(buffer)
}

/// Read from buffer until `\r\n\r\n`. The sequence is included at the end if found,
/// and excluded if the buffer ends before it's complete.
pub fn read_until_double_crlf<R: std::io::Read>(reader: &mut R) -> std::io::Result<Vec<u8>> {
    // Buffer to store the bytes read from the input
    let mut buffer = Vec::new();

    // Temporary buffer to read one byte at a time
    let mut temp = [0u8; 1];

    // Read loop: continue reading until we find the double CRLF sequence (\r\n\r\n)
    while reader.read(&mut temp)? == 1 {
        // Append the byte to our main buffer
        buffer.push(temp[0]);

        // Check if the last four bytes are \r\n\r\n
        let len = buffer.len();
        if len >= 4
            && buffer[len - 4] == b'\r'
            && buffer[len - 3] == b'\n'
            && buffer[len - 2] == b'\r'
            && buffer[len - 1] == b'\n'
        {
            break; // Stop reading once double CRLF is found
        }
    }

    // Return the collected bytes, including the \r\n\r\n if found
    Ok(buffer)
}


#[cfg(test)]
mod crlf_tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn reads_up_to_crlf() {
        let input = b"Hello, world!\r\nRest of the data";
        let mut cursor = Cursor::new(input);

        let result = read_until_crlf(&mut cursor).unwrap();
        assert_eq!(result, b"Hello, world!\r\n");
    }

    #[test]
    fn handles_no_crlf() {
        let input = b"No line ending here";
        let mut cursor = Cursor::new(input);

        let result = read_until_crlf(&mut cursor).unwrap();
        assert_eq!(result, input); // Reads entire input since no \r\n
    }

    #[test]
    fn handles_empty_input() {
        let input = b"";
        let mut cursor = Cursor::new(input);

        let result = read_until_crlf(&mut cursor).unwrap();
        assert_eq!(result, b"");
    }

    #[test]
    fn stops_at_first_crlf() {
        let input = b"Line one\r\nLine two\r\n";
        let mut cursor = Cursor::new(input);

        let result = read_until_crlf(&mut cursor).unwrap();
        assert_eq!(result, b"Line one\r\n");
    }
}

#[cfg(test)]
mod double_crlf_tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn reads_up_to_double_crlf() {
        let input = b"Header line 1\r\nHeader line 2\r\n\r\nBody starts here";
        let mut cursor = Cursor::new(input);

        let result = read_until_double_crlf(&mut cursor).unwrap();
        assert_eq!(result, b"Header line 1\r\nHeader line 2\r\n\r\n");
    }

    #[test]
    fn handles_no_double_crlf() {
        let input = b"Just some text without the sequence";
        let mut cursor = Cursor::new(input);

        let result = read_until_double_crlf(&mut cursor).unwrap();
        assert_eq!(result, input); // Reads entire input since no \r\n\r\n
    }

    #[test]
    fn handles_empty_input() {
        let input = b"";
        let mut cursor = Cursor::new(input);

        let result = read_until_double_crlf(&mut cursor).unwrap();
        assert_eq!(result, b"");
    }

    #[test]
    fn stops_at_first_double_crlf() {
        let input = b"First block\r\n\r\nSecond block\r\n\r\n";
        let mut cursor = Cursor::new(input);

        let result = read_until_double_crlf(&mut cursor).unwrap();
        assert_eq!(result, b"First block\r\n\r\n");
    }

    #[test]
    fn handles_partial_sequence_at_end() {
        let input = b"Almost there\r\n";
        let mut cursor = Cursor::new(input);

        let result = read_until_double_crlf(&mut cursor).unwrap();
        assert_eq!(result, input); // Doesn't find full \r\n\r\n
    }
}


/// Valid for both response and request packets.
/// **NOTE**: Not implemented for HTTP/0.9 (because its response headers have no `\r\n` sequences.
///
/// Fails if not enough bytes read to guarantee a proper packet with the specified length.
pub fn read_full_packet<R: std::io::Read>(reader: &mut R) -> std::io::Result<(String, Option<String>)> {

    use std::io::{Error, ErrorKind};

    let mut header_buffer = Vec::new();
    let mut temp = [0u8; 1];

    // Read until we find \r\n\r\n (end of headers)
    while reader.read(&mut temp)? == 1 {
        header_buffer.push(temp[0]);

        if header_buffer.len() >= 4 && &header_buffer[header_buffer.len() - 4..] == b"\r\n\r\n" {
            break;
        }
    }

    // If we didn't find the header terminator, return an error
    if header_buffer.len() < 4 || &header_buffer[header_buffer.len() - 4..] != b"\r\n\r\n" {
        return Err(Error::new(
            ErrorKind::UnexpectedEof,
            "Ran out of bytes before finding end of headers (\\r\\n\\r\\n)",
        ));
    }

    // Convert header buffer to string for parsing
    let headers_str = String::from_utf8(header_buffer.clone())
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    // Look for Content-Length header
    let body = if let Some(content_length_line) = headers_str
        .lines()
        .find(|line| line.to_ascii_lowercase().starts_with("content-length:"))
    {
        // Extract the numeric part
        let parts: Vec<&str> = content_length_line.splitn(2, ':').collect();
        let size_str = parts.get(1)
            .map(|s| s.trim())
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Malformed Content-Length header"))?;

        // Parse the size
        let content_length: usize = size_str.parse().map_err(|_| {
            Error::new(ErrorKind::InvalidData, "Invalid Content-Length value")
        })?;

        // Read the body
        let mut body_buffer = vec![0u8; content_length];
        let mut total_read = 0;

        while total_read < content_length {
            let bytes_read = reader.read(&mut body_buffer[total_read..])?;
            if bytes_read == 0 {
                return Err(Error::new(
                    ErrorKind::UnexpectedEof,
                    format!(
                        "Expected {} bytes for body, but only received {}",
                        content_length, total_read
                    ),
                ));
            }
            total_read += bytes_read;
        }

        Some(String::from_utf8(body_buffer)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?)
    } else {
        None
    };

    Ok((headers_str, body))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn reads_http_1_0_request_without_body() {
        let request = b"GET /index.html HTTP/1.0\r\nHost: example.com\r\n\r\n";
        let mut cursor = Cursor::new(request);

        let result = read_full_packet(&mut cursor).unwrap();
        assert_eq!(result, (
            String::from_utf8_lossy(request).into_owned(),
            None
        ));
    }

    #[test]
    fn reads_http_1_0_response_with_body() {
        let response = b"HTTP/1.0 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
        let mut cursor = Cursor::new(response);

        let (headers, body) = read_full_packet(&mut cursor).unwrap();
        assert_eq!(headers, "HTTP/1.0 200 OK\r\nContent-Length: 13\r\n\r\n");
        assert_eq!(body, Some("Hello, world!".to_string()));
    }

    #[test]
    fn returns_error_on_incomplete_headers() {
        let incomplete = b"GET / HTTP/1.0\r\nHost: example.com\r\n"; // Missing \r\n\r\n
        let mut cursor = Cursor::new(incomplete);

        let result = read_full_packet(&mut cursor);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn returns_error_on_incomplete_body() {
        let partial_body = b"HTTP/1.0 200 OK\r\nContent-Length: 20\r\n\r\nShort body";
        let mut cursor = Cursor::new(partial_body);

        let result = read_full_packet(&mut cursor);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn handles_request_with_content_length_zero() {
        let request = b"POST /submit HTTP/1.0\r\nContent-Length: 0\r\n\r\n";
        let mut cursor = Cursor::new(request);

        let result = read_full_packet(&mut cursor).unwrap();
        assert_eq!(result, (
            String::from_utf8_lossy(request).into_owned(),
            Some(String::new())
        ));
    }

    #[test]
    fn returns_error_on_invalid_content_length() {
        let bad_header = b"HTTP/1.0 200 OK\r\nContent-Length: notanumber\r\n\r\n";
        let mut cursor = Cursor::new(bad_header);

        let result = read_full_packet(&mut cursor);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidData);
    }
}

