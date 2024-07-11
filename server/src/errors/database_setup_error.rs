use std::fmt;
use std::fs;
use std::io;
use rusqlite;

pub enum DatabaseSetupError {
    IOError(io::Error),
    SqlError(rusqlite::Error)
}

impl fmt::Debug for DatabaseSetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            DatabaseSetupError::IOError(e) => e.to_string(),
            DatabaseSetupError::SqlError(e) => e.to_string()
        };
        write!(f, "{val}")
    }
}

impl fmt::Display for DatabaseSetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            DatabaseSetupError::IOError(e) => e.to_string(),
            DatabaseSetupError::SqlError(e) => e.to_string()
        };
        write!(f, "{val}")
    }
}