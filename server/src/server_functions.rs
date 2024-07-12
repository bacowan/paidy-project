use rusqlite::{ params_from_iter, Error, ErrorCode, Result };
use rand::Rng;

use crate::errors::server_error::ServerError;
use crate::database_connector::DatabaseConnector;
use crate::{ rest_responses, rest_bodies };

pub fn setup_database(connector: &dyn DatabaseConnector) -> Result<(), ServerError> {
    let connection = connector.open().sql_err()?;

    // Table setup
    connection.execute("
        CREATE TABLE IF NOT EXISTS menu_items (
            id INTEGER PRIMARY KEY,
            name TEXT UNIQUE);", ()).sql_err()?;
    connection.execute("
        CREATE TABLE IF NOT EXISTS orders (
            id INTEGER PRIMARY KEY,
            menu_item_id INTEGER,
            table_number INTEGER,
            minutes_to_cook INTEGER,
            FOREIGN KEY(menu_item_id) REFERENCES menu_items(id));", ()).sql_err()?;
    connection.execute("
        CREATE TABLE IF NOT EXISTS idempotent_requests (
            idempotency_key TEXT PRIMARY KEY);", ()).sql_err()?;

    // Data setup
    let menu_items_exist_query = "SELECT * FROM menu_items;";
    let mut stmt = connection.prepare(menu_items_exist_query).sql_err()?;
    let menu_items_exist = stmt.exists([]).sql_err()?;

    if !menu_items_exist {
        let data_sql = "
            INSERT INTO menu_items (name) VALUES
                ('Hamburger'), ('Salad'), ('Sushi'), ('Ice Cream'), ('Soda');";

        connection.execute(data_sql, ()).sql_err()?;
    }

    Result::Ok(())
}

pub fn get_menu_items(connector: &dyn DatabaseConnector) -> Result<rest_responses::MenuItems, ServerError> {
    let connection = connector.open().sql_err()?;
    let query = "SELECT id, name FROM menu_items";
    let mut stmt = connection.prepare(query).sql_err()?;
    let query_result = stmt.query_map(
        [],
        |row| Result::Ok(rest_responses::MenuItem {
            id: row.get(0)?,
            name: row.get(1)?
        })).sql_err()?;

    let mut items = Vec::new();
    for item in query_result {
        items.push(item.sql_err()?);
    }

    Result::Ok(
        rest_responses::MenuItems {
            menu_items: items
        }
    )
}

pub fn add_orders(connector: &dyn DatabaseConnector, table_number: u32, orders: rest_bodies::Orders) -> Result<rest_responses::Orders, ServerError> {
    let mut connection = connector.open().sql_err()?;
    let transaction = connection.transaction().sql_err()?;

    // if an idempotency key was required, check that first. On conflict, an Err will be returned.
    if let Some(key) = orders.idempotency_key {
        transaction.execute(
            "INSERT INTO idempotent_requests (idempotency_key) VALUES (:key)",
            &[(":key", &key)])
            .map_err(|e| match e {
                Error::SqliteFailure(err, _) if err.code == ErrorCode::ConstraintViolation =>
                    ServerError::Idempotency,
                x => ServerError::SqlError(x)
            })?;
    }

    let mut ids = Vec::new();

    // add orders one at a time
    for order in &orders.orders {
        let cook_time = rand::thread_rng().gen_range(5..15);
        let rows = transaction.execute(
            "INSERT INTO orders (menu_item_id, table_number, minutes_to_cook)
            SELECT
                m.id,
                :table_number,
                :cook_time
            FROM menu_items AS m WHERE m.id = :menu_item_id",
            &[
                (":table_number", &table_number.to_string()),
                (":cook_time", &cook_time.to_string()),
                (":menu_item_id", &order.menu_item_id.to_string())])
            .sql_err()?;
        
        // if the requested menu item does not exist, no data will be added; return an error.
        if rows == 0 {
            return Err(ServerError::DataNotFound);
        }

        // keep track of all added ids so they can be queried after the transaction
        ids.push(transaction.last_insert_rowid());
    };

    transaction.commit().sql_err()?;

    // query the added items for the response
    let query = format!("SELECT o.id, o.minutes_to_cook, m.id, m.name
        FROM orders AS o
        INNER JOIN menu_items AS m ON m.id = o.menu_item_id
        WHERE o.id IN ({})",
        (1..ids.len() + 1).map(|x| format!("?{x}")).collect::<Vec<_>>().join(","));

    let mut stmt = connection.prepare(&query).sql_err()?;
    let query_result = stmt.query_map(
        params_from_iter(ids.iter().map(|i| i.to_string())),
        |row| Result::Ok(rest_responses::Order {
            id: row.get(0)?,
            minutes_to_cook: row.get(1)?,
            menu_item_id: row.get(2)?,
            menu_item_name: row.get(3)?
        })).sql_err()?;


    let mut items = Vec::new();
    for item in query_result {
        items.push(item.sql_err()?);
    }

    Result::Ok(
        rest_responses::Orders {
            orders: items
        }
    )
}

pub fn get_orders(connector: &dyn DatabaseConnector, table_number: u32) -> Result<rest_responses::Orders, ServerError> {
    let connection = connector.open().sql_err()?;
    let mut stmt = connection.prepare(
        "SELECT o.id, o.minutes_to_cook, m.id, m.name
        FROM orders AS o
        INNER JOIN menu_items AS m ON m.id = o.menu_item_id
        WHERE o.table_number = :table_number").sql_err()?;
    let query_result = stmt.query_map(
        &[(":table_number", &table_number.to_string())],
        |row| Result::Ok(rest_responses::Order {
            id: row.get(0)?,
            minutes_to_cook: row.get(1)?,
            menu_item_id: row.get(2)?,
            menu_item_name: row.get(3)?
        })).sql_err()?;

    let mut items = Vec::new();
    for item in query_result {
        items.push(item.sql_err()?);
    }

    Result::Ok(
        rest_responses::Orders {
            orders: items
        }
    )
}

pub fn get_order(connector: &dyn DatabaseConnector, table_number: u32, order_id: u32) -> Result<rest_responses::Order, ServerError> {
    let connection = connector.open().sql_err()?;
    let mut stmt = connection.prepare(
        "SELECT o.id, o.minutes_to_cook, m.id, m.name
        FROM orders AS o
        INNER JOIN menu_items AS m ON m.id = o.menu_item_id
        WHERE o.id = :order_id
        AND o.table_number = :table_number").sql_err()?;
    let query_result = stmt.query_row(
        &[
            (":order_id", &order_id.to_string()),
            (":table_number", &table_number.to_string())],
        |row| Result::Ok(rest_responses::Order {
            id: row.get(0)?,
            minutes_to_cook: row.get(1)?,
            menu_item_id: row.get(2)?,
            menu_item_name: row.get(3)?
        }))
        .map_err(|e| match e {
            Error::QueryReturnedNoRows => ServerError::DataNotFound,
            x => ServerError::SqlError(x)
        })?;

    Result::Ok(query_result)
}

pub fn delete_order(connector: &dyn DatabaseConnector, table_number: u32, order_id: u32) -> Result<(), ServerError> {
    let connection = connector.open().sql_err()?;
    connection.execute(
        "DELETE FROM orders
            WHERE id = :order_id
            AND table_number = :table_number",
        &[
            (":order_id", &order_id.to_string()),
            (":table_number", &table_number.to_string())]).sql_err()?;
    Result::Ok(())
}

pub trait DisplayResultMethods<T> {
    fn sql_err(self) -> Result<T, ServerError>;
}

impl<T> DisplayResultMethods<T> for Result<T, rusqlite::Error>
{
    // Convenience function for calling map_err on a Result<_, rusqlite::Error>
    // that converts the error to a ServerError as defined by this application
    fn sql_err(self) -> Result<T, ServerError>
    {
        self.map_err(|e| ServerError::SqlError(e))
    }
}