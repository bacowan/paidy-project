use std::fmt;

pub enum ServerError {
    NoRowsReturned(),
    DataNotFound(),
    Idempotency(),
    SqlError(String)
}

impl fmt::Debug for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            ServerError::NoRowsReturned() => "NoRowsReturned",
            ServerError::DataNotFound() => "DataNotFound",
            ServerError::Idempotency() => "Idempotency",
            ServerError::SqlError(e) => &format!("SqlError: {e}")
        };
        write!(f, "{val}")
    }
}