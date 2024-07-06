#[derive(Debug)]
pub enum ServerError {
    NoRowsReturned(),
    DataNotFound(),
    Idempotency(),
    StringError(String)
}