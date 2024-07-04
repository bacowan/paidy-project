use rusqlite::{ Connection, Result };

pub trait DatabaseConnection {
    fn open(path: String) -> Result<(), rusqlite::Error> {
        Connection::open(path)
    }
}