use rusqlite::{ Connection, Result };

pub trait DatabaseConnector {
    fn open(&self) -> Result<rusqlite::Connection, rusqlite::Error>;
}

pub struct DefaultDatabaseConnector {
    pub path: String
}

impl DatabaseConnector for DefaultDatabaseConnector {
    fn open(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        Connection::open(self.path.clone())
    }
}