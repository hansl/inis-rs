use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("INI parse error: {0}")]
    ParseError(String),

    #[error("Deserialization error: {0}")]
    Message(String),
}

impl serde::de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}
