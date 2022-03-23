use thiserror::Error;

/// Error generated reading and writing binary data.
#[derive(Debug, Error)]
pub enum BinaryError {
    /// Error generated attempted to read past the end of file.
    #[error("attempt to read past EOF")]
    ReadPastEof,
    /// Error generated converting to UTF-8.
    #[error(transparent)]
    Utf8Error(#[from] std::string::FromUtf8Error),
    /// Error generated by input / output.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
