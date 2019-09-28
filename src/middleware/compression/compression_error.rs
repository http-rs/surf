use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub(crate) enum CompressionError {
    InvalidContentEncodingHeader,
    UnsupportedContentEncoding,
}

impl Error for CompressionError {}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompressionError::InvalidContentEncodingHeader => {
                f.write_str("Invalid content encodinig header")
            }
            CompressionError::UnsupportedContentEncoding => {
                f.write_str("Unsupported content encoding")
            }
        }
    }
}
