

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use server::database_connection::DatabaseConnector;
    use server::server_functions;
    use server::server_errors::ServerError;


    #[test]
    fn test_get_orders_database_error() {
        let connection = TestDatabaseConnector{};
        let result = server_functions::get_orders(&connection, 1);
        assert!(result.is_err(), "Database was not initialized, so this should fail");
        assert!(matches!(result.err().unwrap(), ServerError::SqlError(_)), "Error is unexpected type");
    }

    struct TestDatabaseConnector {}
    
    impl DatabaseConnector for TestDatabaseConnector {
        fn open(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
            Connection::open_in_memory()
        }
    }
}