use rusqlite::{ Connection, Result };
use rocket::serde::{ Serialize };

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
                FOREIGN KEY(menu_item_id) REFERENCES menu_items(internal_id));", ())?;
    }
    Result::Ok(())
}

pub fn table_orders(table_id: &str) -> &str {
    return "HI"
}

pub fn menu_items(connector: &impl DatabaseConnector) -> Result<MenuItemResponse, rusqlite::Error> {
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