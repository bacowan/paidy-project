use rocket::{ get, post, delete, routes, launch };
use rusqlite::{ Connection, Result };
use rocket::http::{ Status, ContentType };
use rocket::serde::json::{ Json, to_string };
use rocket::serde::Deserialize;
use server::rest_bodies;

mod server_functions;
mod database_connection;

const DATABASE_PATH: &str = "database.db";

#[get("/tables/<table_number>/orders")]
fn get_table_orders(table_number: u32) -> (Status, (ContentType, String)) {
    match server_functions::get_orders(&get_connector(), table_number) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(item_string) => (Status::Ok, (ContentType::JSON, item_string)),
            Result::Err(e) => (Status::BadRequest, (ContentType::JSON, format!("{{ error: \"{e}\"}}").to_string()))
        },
        Result::Err(e) => (Status::BadRequest, (ContentType::JSON, format!("{{ error: {}}}", e.to_string()).to_string()))
    }
}

#[post("/tables/<table_id>/orders", format = "json", data = "<orders_data>")]
fn post_table_order(table_id: u32, orders_data: Json<rest_bodies::Orders>) -> (Status, (ContentType, String)) {
    match server_functions::add_orders(&get_connector(), table_id, orders_data.into_inner()) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(item_string) => (Status::Ok, (ContentType::JSON, "{}".to_string())),
            Result::Err(e) => (Status::BadRequest, (ContentType::JSON, format!("{{ error: \"{e}\"}}").to_string()))
        },
        Result::Err(e) => (Status::BadRequest, (ContentType::JSON, format!("{{ error: \"{e}\"}}").to_string()))
    }
}

#[get("/orders/<order_id>")]
fn get_table_order(order_id: u32) -> (Status, (ContentType, String)) {
    match server_functions::get_order(&get_connector(), order_id) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(item_string) => (Status::Ok, (ContentType::JSON, item_string)),
            Result::Err(e) => (Status::BadRequest, (ContentType::JSON, format!("{{ error: \"{e}\"}}").to_string()))
        },
        Result::Err(e) => (Status::BadRequest, (ContentType::JSON, format!("{{ error: {}}}", e.to_string()).to_string()))
    }
}

#[delete("/orders/<order_id>")]
fn delete_table_order(order_id: u32) -> (Status, (ContentType, String)) {
    match server_functions::delete_order(&get_connector(), order_id) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(_) => (Status::Ok, (ContentType::JSON, "{}".to_string())),
            Result::Err(e) => (Status::BadRequest, (ContentType::JSON, format!("{{ error: \"{e}\"}}").to_string()))
        },
        Result::Err(e) => (Status::BadRequest, (ContentType::JSON, format!("{{ error: {}}}", e.to_string()).to_string()))
    }
}

#[get("/menu-items")]
fn get_menu_items() -> (Status, (ContentType, String)) {
    match server_functions::get_menu_items(&get_connector()) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(item_string) => (Status::Ok, (ContentType::JSON, item_string)),
            Result::Err(e) => (Status::BadRequest, (ContentType::JSON, format!("{{ error: \"{e}\"}}").to_string()))
        },
        Result::Err(e) => (Status::BadRequest, (ContentType::JSON, format!("{{ error: \"{e}\"}}").to_string()))
    }
}

#[launch]
fn rocket() -> _ {
    match server_functions::setup_database(&get_connector()) {
        Ok(_) => {},
        Err(err) => panic!("Failed to setup database: {:?}", err),
    };
    rocket::build()
        .mount("/", routes![get_table_orders])
        .mount("/", routes![post_table_order])
        .mount("/", routes![get_table_order])
        .mount("/", routes![delete_table_order])
        .mount("/", routes![get_menu_items])
}

fn get_connector() -> database_connection::DefaultDatabaseConnector {
    database_connection::DefaultDatabaseConnector { path: DATABASE_PATH.to_string() }
}