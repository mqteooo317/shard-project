use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ShardError {
    Io(io::Error),
    Cache(String),
    Fragment(String),
    Config(String),
    Backend(String),
}

impl fmt::Display for ShardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShardError::Io(e) => write!(f, "IO error: {}", e),
            ShardError::Cache(s) => write!(f, "Cache error: {}", s),
            ShardError::Fragment(s) => write!(f, "Fragment error: {}", s),
            ShardError::Config(s) => write!(f, "Config error: {}", s),
            ShardError::Backend(s) => write!(f, "Backend error: {}", s),
        }
    }
}

impl std::error::Error for ShardError {}

impl From<io::Error> for ShardError {
    fn from(e: io::Error) -> Self {
        ShardError::Io(e)
    }
}