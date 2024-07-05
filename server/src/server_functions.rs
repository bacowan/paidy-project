use rusqlite::{ Connection, Result };
use rocket::serde::{ Serialize, Deserialize };
use rand::Rng;
use std::fmt::Display;

use crate::database_connection::DatabaseConnector;
use server::{ rest_responses, rest_bodies };

pub fn setup_database(connector: &impl DatabaseConnector) -> Result<(), rusqlite::Error> {
    let connection = connector.open()?;
    
    let menu_items_exists_query = "SELECT 1 FROM sqlite_master WHERE type='table' AND name='menu_items';";
    let mut stmt = connection.prepare(menu_items_exists_query)?;
    let menu_items_table_exists = stmt.exists([])?;

    if !menu_items_table_exists {
        connection.execute("
            CREATE TABLE menu_items (
                id INTEGER PRIMARY KEY,
                name TEXT);", ())?;
            connection.execute("INSERT INTO menu_items (name) VALUES ('Hamburger')", ())?;
            connection.execute("INSERT INTO menu_items (name) VALUES ('Salad')", ())?;
            connection.execute("INSERT INTO menu_items (name) VALUES ('Sushi')", ())?;
            connection.execute("INSERT INTO menu_items (name) VALUES ('Ice Cream')", ())?;
            connection.execute("INSERT INTO menu_items (name) VALUES ('Soda');", ())?;
    }

    let orders_exists_query = "SELECT 1 FROM sqlite_master WHERE type='table' AND name='orders';";
    let mut stmt = connection.prepare(orders_exists_query)?;
    let orders_table_exists = stmt.exists([])?;

    if !orders_table_exists {
        connection.execute("
            CREATE TABLE orders (
                id INTEGER PRIMARY KEY,
                idempotency_key TEXT UNIQUE,
                menu_item_id INTEGER,
                table_number INTEGER,
                minutes_to_cook INTEGER,
                FOREIGN KEY(menu_item_id) REFERENCES menu_items(id));", ())?;
    }
    Result::Ok(())
}

pub fn get_menu_items(connector: &impl DatabaseConnector) -> Result<rest_responses::MenuItems, rusqlite::Error> {
    let connection = connector.open()?;
    let query = "SELECT id, name FROM menu_items";
    let mut stmt = connection.prepare(query)?;
    let query_result = stmt.query_map(
        [],
        |row| Result::Ok(rest_responses::MenuItem {
            id: row.get(0)?,
            name: row.get(1)?
        }))?;

    let mut items = Vec::new();
    for item in query_result {
        items.push(item?);
    }

    Result::Ok(
        rest_responses::MenuItems {
            menu_items: items
        }
    )
}

pub fn add_orders(connector: &impl DatabaseConnector, table_number: u32, orders: rest_bodies::Orders) -> Result<(), String> {
    let mut connection = connector.open().str_err()?;
    let transaction = connection.transaction().str_err()?;

    for order in &orders.orders {
        let cook_time = rand::thread_rng().gen_range(5..15);
        let rows = transaction.execute(
            "INSERT INTO orders (idempotency_key, menu_item_id, table_number, minutes_to_cook)
            SELECT
                :idempotency_key,
                m.id,
                :table_number,
                :cook_time
            FROM menu_items AS m WHERE m.id = :menu_item_id",
            &[
                (":idempotency_key", &order.idempotency_key.to_string()),
                (":table_number", &table_number.to_string()),
                (":cook_time", &cook_time.to_string()),
                (":menu_item_id", &order.menu_item_id.to_string())])
            .str_err()?;
        if rows == 0 {
            return Err("No rows added; menu item likely does not exist".to_string());
        }
    };

    transaction.commit().str_err()?;
    
    Result::Ok(())
}

pub fn get_orders(connector: &impl DatabaseConnector, table_number: u32) -> Result<rest_responses::Orders, rusqlite::Error> {
    let connection = connector.open()?;
    let mut stmt = connection.prepare(
        "SELECT o.id, o.minutes_to_cook, m.id, m.name
        FROM orders AS o
        INNER JOIN menu_items AS m ON m.id = o.menu_item_id
        WHERE o.table_number = :table_number")?;
    let query_result = stmt.query_map(
        &[(":table_number", &table_number.to_string())],
        |row| Result::Ok(rest_responses::Order {
            id: row.get(0)?,
            minutes_to_cook: row.get(1)?,
            menu_item_id: row.get(2)?,
            menu_item_name: row.get(3)?
        }))?;

    let mut items = Vec::new();
    for item in query_result {
        items.push(item?);
    }

    Result::Ok(
        rest_responses::Orders {
            orders: items
        }
    )
}

pub fn get_order(connector: &impl DatabaseConnector, order_id: u32) -> Result<rest_responses::Order, rusqlite::Error> {
    let connection = connector.open()?;
    let mut stmt = connection.prepare(
        "SELECT o.id, o.minutes_to_cook, m.id, m.name
        FROM orders AS o
        INNER JOIN menu_items AS m ON m.id = o.menu_item_id
        WHERE o.id = :order_id")?;
    let query_result = stmt.query_row(
        &[(":order_id", &order_id.to_string())],
        |row| Result::Ok(rest_responses::Order {
            id: row.get(0)?,
            minutes_to_cook: row.get(1)?,
            menu_item_id: row.get(2)?,
            menu_item_name: row.get(3)?
        }))?;

    Result::Ok(query_result)
}

pub fn delete_order(connector: &impl DatabaseConnector, order_id: u32) -> Result<(), rusqlite::Error> {
    let connection = connector.open()?;
    connection.execute(
        "DELETE FROM orders
            WHERE id = :order_id",
        &[(":order_id", &order_id.to_string())])?;
    Result::Ok(())
}

pub trait ResultExtensionMethods<T, E> {
    fn str_err(self) -> Result<T, String>;
}

impl<T, E> ResultExtensionMethods<T, E> for Result<T, E>
where
    E: Display,
{
    fn str_err(self) -> Result<T, String>
    {
        self.map_err(|e| e.to_string())
    }
}