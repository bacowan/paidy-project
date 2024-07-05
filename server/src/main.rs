use rocket::{ get, post, delete, routes, launch };
use rusqlite::{ Connection, Result };
use rocket::http::{ Status, ContentType };
use rocket::serde::json::{ Json, to_string };
use rocket::serde::Deserialize;
use server_functions::OrdersPostData;

mod server_functions;

const DATABASE_PATH: &str = "database.db";

#[get("/orders/<table_number>")]
fn get_table_orders(table_number: u32) -> (Status, (ContentType, String)) {
    match server_functions::get_orders(&get_connector(), table_number) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(item_string) => (Status::Ok, (ContentType::JSON, item_string)),
            Result::Err(e) => (Status::BadRequest, (ContentType::JSON, "{ error: \"test\"}".to_string()))
        },
        Result::Err(e) => (Status::BadRequest, (ContentType::JSON, format!("{{ error: {}}}", e.to_string()).to_string()))
    }
}

#[post("/orders/<table_id>", format = "json", data = "<orders_data>")]
fn post_table_order(table_id: u32, orders_data: Json<OrdersPostData>) -> (Status, (ContentType, String)) {
    match server_functions::add_orders(&get_connector(), table_id, orders_data.into_inner()) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(item_string) => (Status::Ok, (ContentType::JSON, "{}".to_string())),
            Result::Err(e) => (Status::BadRequest, (ContentType::JSON, "{ error: \"test\"}".to_string()))
        },
        Result::Err(e) => (Status::BadRequest, (ContentType::JSON, "{ error: \"test\"}".to_string()))
    }
}

#[get("/orders/<table_number>/<order_id>")]
fn get_table_order(table_number: u32, order_id: u32) -> (Status, (ContentType, String)) {
    match server_functions::get_order(&get_connector(), table_number, order_id) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(item_string) => (Status::Ok, (ContentType::JSON, item_string)),
            Result::Err(e) => (Status::BadRequest, (ContentType::JSON, "{ error: \"test\"}".to_string()))
        },
        Result::Err(e) => (Status::BadRequest, (ContentType::JSON, format!("{{ error: {}}}", e.to_string()).to_string()))
    }
}

#[delete("/orders/<table_number>/<order_id>")]
fn delete_table_order(table_number: u32, order_id: u32) -> (Status, (ContentType, String)) {
    match server_functions::delete_order(&get_connector(), table_number, order_id) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(_) => (Status::Ok, (ContentType::JSON, "{}".to_string())),
            Result::Err(e) => (Status::BadRequest, (ContentType::JSON, "{ error: \"test\"}".to_string()))
        },
        Result::Err(e) => (Status::BadRequest, (ContentType::JSON, format!("{{ error: {}}}", e.to_string()).to_string()))
    }
}

#[get("/menu-items")]
fn get_menu_items() -> (Status, (ContentType, String)) {
    match server_functions::get_menu_items(&get_connector()) {
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
        .mount("/", routes![get_table_orders])
        .mount("/", routes![post_table_order])
        .mount("/", routes![get_table_order])
        .mount("/", routes![delete_table_order])
        .mount("/", routes![get_menu_items])
}

fn get_connector() -> server_functions::DefaultDatabaseConnector {
    server_functions::DefaultDatabaseConnector { path: DATABASE_PATH.to_string() }
}