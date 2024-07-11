use std::fmt;
use rusqlite;

pub enum ServerError {
    NoRowsReturned,
    DataNotFound,
    Idempotency,
    SqlError(rusqlite::Error)
}

impl fmt::Debug for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            ServerError::NoRowsReturned => "NoRowsReturned",
            ServerError::DataNotFound => "DataNotFound",
            ServerError::Idempotency => "Idempotency",
            ServerError::SqlError(e) => &e.to_string()
        };
        write!(f, "{val}")
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            ServerError::NoRowsReturned => "NoRowsReturned",
            ServerError::DataNotFound => "DataNotFound",
            ServerError::Idempotency => "Idempotency",
            ServerError::SqlError(e) => &e.to_string()
        };
        write!(f, "{val}")
    }
}