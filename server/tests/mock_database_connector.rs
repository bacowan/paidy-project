use std::path::{Path, PathBuf};

use server::database_connection::DatabaseConnector;
use rusqlite::{Connection, OpenFlags};
use tempfile::NamedTempFile;

pub fn new() -> Result<MockDatabaseConnector, String> {
    let temp_file = NamedTempFile::new().map_err(|e| e.to_string())?;
    Ok(MockDatabaseConnector {
        temp_file_path: temp_file.path().to_path_buf()
    })
}

pub struct MockDatabaseConnector {
    temp_file_path: PathBuf
}

impl DatabaseConnector for MockDatabaseConnector {
    fn open(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        Connection::open(&self.temp_file_path)
    }
}