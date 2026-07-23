#![forbid(unsafe_code)]

pub mod cdb;
pub mod name;
pub mod packet;
pub mod server;
pub mod zone;

pub use name::Name;
pub use packet::{Message, Question, RData, Record, RecordType};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Format(&'static str),
    InvalidName(String),
    InvalidRecord(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Format(s) => write!(f, "DNS format error: {s}"),
            Self::InvalidName(s) => write!(f, "invalid DNS name: {s}"),
            Self::InvalidRecord(s) => write!(f, "invalid tinydns record: {s}"),
        }
    }
}
impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
pub type Result<T> = std::result::Result<T, Error>;
