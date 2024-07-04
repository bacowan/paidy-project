use rocket::{ get, routes, launch };
use rusqlite::{ Connection, Result };
use rocket::http::{ Status, ContentType };
use rocket::serde::json::{ Json, to_string };

mod server_functions;

const DATABASE_PATH: &str = "database.db";

#[get("/orders/<table_id>")]
fn table_orders(table_id: &str) -> &'static str {
    return "HI"
}

#[get("/menu-items")]
fn menu_items() -> (Status, (ContentType, String)) {
    match server_functions::menu_items(&get_connector()) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(item_string) => (Status::Ok, (ContentType::JSON, item_string)),
            Result::Err(e) => (Status::BadRequest, (ContentType::JSON, "{ error: \"test\"}".to_string()))
        },
        Result::Err(e) => (Status::BadRequest, (ContentType::JSON, "{ error: \"test\"}".to_string()))
    }
}

#[launch]
fn rocket() -> _ {
    match server_functions::setup_database(&get_connector()) {
        Ok(_) => {},
        Err(err) => panic!("Failed to setup database: {:?}", err),
    };
    rocket::build()
        .mount("/", routes![table_orders])
        .mount("/", routes![menu_items])
}

fn get_connector() -> server_functions::DefaultDatabaseConnector {
    server_functions::DefaultDatabaseConnector { path: DATABASE_PATH.to_string() }
}