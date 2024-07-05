use rusqlite::{ Connection, Result };
use rocket::serde::{ Serialize, Deserialize };
use rand::Rng;

pub fn setup_database(connector: &impl DatabaseConnector) -> Result<(), rusqlite::Error> {
    let connection = connector.open()?;
    
    let menu_items_exists_query = "SELECT 1 FROM sqlite_master WHERE type='table' AND name='menu_items';";
    let mut stmt = connection.prepare(menu_items_exists_query)?;
    let menu_items_table_exists = stmt.exists([])?;

    if !menu_items_table_exists {
        connection.execute("
            CREATE TABLE menu_items (
                internal_id INTEGER PRIMARY KEY,
                id INTEGER UNIQUE,
                name TEXT);", ())?;
            connection.execute("INSERT INTO menu_items (id, name) VALUES (1, 'Hamburger')", ())?;
            connection.execute("INSERT INTO menu_items (id, name) VALUES (2, 'Salad')", ())?;
            connection.execute("INSERT INTO menu_items (id, name) VALUES (3, 'Sushi')", ())?;
            connection.execute("INSERT INTO menu_items (id, name) VALUES (4, 'Ice Cream')", ())?;
            connection.execute("INSERT INTO menu_items (id, name) VALUES (5, 'Soda');", ())?;
    }

    let orders_exists_query = "SELECT 1 FROM sqlite_master WHERE type='table' AND name='orders';";
    let mut stmt = connection.prepare(orders_exists_query)?;
    let orders_table_exists = stmt.exists([])?;

    if !orders_table_exists {
        connection.execute("
            CREATE TABLE orders (
                internal_id INTEGER PRIMARY KEY,
                id INTEGER UNIQUE,
                menu_item_id INTEGER,
                table_number INTEGER,
                minutes_to_cook INTEGER,
                FOREIGN KEY(menu_item_id) REFERENCES menu_items(internal_id),
                UNIQUE (id, table_number) ON CONFLICT ABORT);", ())?;
    }
    Result::Ok(())
}

pub fn get_menu_items(connector: &impl DatabaseConnector) -> Result<MenuItemResponse, rusqlite::Error> {
    let connection = connector.open()?;
    let query = "SELECT id, name FROM menu_items";
    let mut stmt = connection.prepare(query)?;
    let query_result = stmt.query_map(
        [],
        |row| Result::Ok(MenuItem {
            id: row.get(0)?,
            name: row.get(1)?
        }))?;

    let mut items = Vec::new();
    for item in query_result {
        items.push(item?);
    }

    Result::Ok(
        MenuItemResponse {
            menu_items: items
        }
    )
}

pub fn add_orders(connector: &impl DatabaseConnector, table_id: u32, orders: OrdersPostData) -> Result<(), rusqlite::Error> {
    let mut connection = connector.open()?;
    let transaction = connection.transaction()?;

    for order in &orders.orders {
        let cook_time = rand::thread_rng().gen_range(5..15);
        transaction.execute(
            "INSERT INTO orders (id, menu_item_id, table_number, minutes_to_cook)
            SELECT :order_id, m.internal_id, :table_id, :cook_time FROM menu_items AS m WHERE m.id = :menu_item_id",
            &[
                (":order_id", &order.order_id.to_string()),
                (":table_id", &table_id.to_string()),
                (":cook_time", &cook_time.to_string()),
                (":menu_item_id", &order.menu_item_id.to_string())])?;
    };

    transaction.commit()?;

    Result::Ok(())
}

pub fn get_orders(connector: &impl DatabaseConnector, table_number: u32) -> Result<OrdersResponse, rusqlite::Error> {
    let connection = connector.open()?;
    let mut stmt = connection.prepare(
        "SELECT o.id, o.minutes_to_cook, m.id, m.name
        FROM orders AS o
        INNER JOIN menu_items AS m ON m.internal_id = o.menu_item_id
        WHERE o.table_number = :table_number")?;
    let query_result = stmt.query_map(
        &[(":table_number", &table_number.to_string())],
        |row| Result::Ok(OrderResponse {
            order_id: row.get(0)?,
            minutes_to_cook: row.get(1)?,
            menu_item_id: row.get(2)?,
            menu_item_name: row.get(3)?
        }))?;

    let mut items = Vec::new();
    for item in query_result {
        items.push(item?);
    }

    Result::Ok(
        OrdersResponse {
            orders: items
        }
    )
}

pub fn get_order(connector: &impl DatabaseConnector, table_number: u32, order_id: u32) -> Result<OrderResponse, rusqlite::Error> {
    let connection = connector.open()?;
    let mut stmt = connection.prepare(
        "SELECT o.id, o.minutes_to_cook, m.id, m.name
        FROM orders AS o
        INNER JOIN menu_items AS m ON m.internal_id = o.menu_item_id
        WHERE o.table_number = :table_number
        AND o.id = :order_id")?;
    let query_result = stmt.query_row(
        &[(":table_number", &table_number.to_string()), (":order_id", &order_id.to_string())],
        |row| Result::Ok(OrderResponse {
            order_id: row.get(0)?,
            minutes_to_cook: row.get(1)?,
            menu_item_id: row.get(2)?,
            menu_item_name: row.get(3)?
        }))?;

    Result::Ok(query_result)
}

pub fn delete_order(connector: &impl DatabaseConnector, table_number: u32, order_id: u32) -> Result<(), rusqlite::Error> {
    let connection = connector.open()?;
    connection.execute(
        "DELETE FROM orders
            WHERE table_number = :table_number AND id = :order_id",
        &[
            (":table_number", &table_number.to_string()),
            (":order_id", &order_id.to_string())])?;
    Result::Ok(())
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MenuItem {
    id: u32,
    name: String
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MenuItemResponse {
    menu_items: Vec<MenuItem>
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct OrdersResponse {
    orders: Vec<OrderResponse>
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct OrderResponse {
    order_id: u32,
    menu_item_id: u32,
    menu_item_name: String,
    minutes_to_cook: u32
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OrderPostData {
    order_id: u32,
    menu_item_id: u32
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OrdersPostData {
    orders: Vec<OrderPostData>
}

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