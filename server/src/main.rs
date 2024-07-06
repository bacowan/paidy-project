#[macro_use] extern crate rocket;

use rocket::{ get, post, delete, routes, launch };
use rusqlite::{ Result };
use rocket::http::{ Status, ContentType };
use rocket::serde::json::{ Json, to_string };
use server::rest_bodies;
use server_errors::ServerError;
use rocket::Request;

use server::server_errors;
use server::server_functions;
use server::database_connection;

const DATABASE_PATH: &str = "database.db";

#[get("/tables/<table_number>/orders")]
fn get_table_orders(table_number: u32) -> (Status, (ContentType, String)) {
    match server_functions::get_orders(&get_connector(), table_number) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(item_string) => (Status::Ok, (ContentType::JSON, item_string)),
            Result::Err(_) => (Status::InternalServerError, (ContentType::JSON, "{ error: \"Server error. Failed to get data.\" }".to_string()))
        },
        Result::Err(_) => (Status::InternalServerError, (ContentType::JSON, "{ error: \"Server error. Failed to get data.\" }".to_string()))
    }
}

#[post("/tables/<table_id>/orders", format = "json", data = "<orders_data>")]
fn post_table_order(table_id: u32, orders_data: Json<rest_bodies::Orders>) -> (Status, (ContentType, String)) {
    match server_functions::add_orders(&get_connector(), table_id, orders_data.into_inner()) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(item_string) => (Status::Ok, (ContentType::JSON, item_string)),
            Result::Err(_) => (Status::InternalServerError, (ContentType::JSON, "{ error: \"Server error. Failed to add data.\" }".to_string()))
        },
        Result::Err(e) => match e {
            ServerError::Idempotency() => (Status::Conflict, (ContentType::JSON, "{ error: \"This order has already been added.\" }".to_string())),
            ServerError::DataNotFound() => (Status::UnprocessableEntity, (ContentType::JSON, "{ error: \"The provided menu_item_id does not exist.\" }".to_string())),
            _ => (Status::InternalServerError, (ContentType::JSON, "{ error: \"Server error. Failed to add data.\" }".to_string()))
        }
    }
}

#[get("/tables/<table_number>/orders/<order_id>")]
fn get_table_order(table_number: u32, order_id: u32) -> (Status, (ContentType, String)) {
    match server_functions::get_order(&get_connector(), table_number, order_id) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(item_string) => (Status::Ok, (ContentType::JSON, item_string)),
            Result::Err(_) => (Status::InternalServerError, (ContentType::JSON, "{ error: \"Server error. Failed to get data.\" }".to_string()))
        },
        Result::Err(e) => match e {
            ServerError::DataNotFound() => (Status::NotFound, (ContentType::JSON, "{ error: \"Provided order does not exist for provided table.\" }".to_string())),
            _ => (Status::InternalServerError, (ContentType::JSON, "{ error: \"Server error. Failed to get data.\" }".to_string()))
        }
    }
}

#[delete("/tables/<table_number>/orders/<order_id>")]
fn delete_table_order(table_number: u32, order_id: u32) -> (Status, (ContentType, String)) {
    match server_functions::delete_order(&get_connector(), table_number, order_id) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(_) => (Status::Ok, (ContentType::JSON, "{}".to_string())),
            Result::Err(_) => (Status::InternalServerError, (ContentType::JSON, "{ error: \"Server error. Failed to delete data.\" }".to_string()))
        },
        Result::Err(_) => (Status::InternalServerError, (ContentType::JSON, "{ error: \"Server error. Failed to delete data.\" }".to_string()))
    }
}

#[get("/menu-items")]
fn get_menu_items() -> (Status, (ContentType, String)) {
    match server_functions::get_menu_items(&get_connector()) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(item_string) => (Status::Ok, (ContentType::JSON, item_string)),
            Result::Err(_) => (Status::InternalServerError, (ContentType::JSON, "{ error: \"Server error. Failed to get data.\" }".to_string()))
        },
        Result::Err(_) => (Status::InternalServerError, (ContentType::JSON, "{ error: \"Server error. Failed to get data.\" }".to_string()))
    }
}

#[catch(400)]
fn internal_error() -> &'static str {
    "{ error: \"Request format could not be understood\" }"
}

#[catch(404)]
fn not_found(req: &Request) -> &'static str {
    "{ error: \"Resource could not be found\" }"
}

#[catch(default)]
fn default(status: Status, req: &Request) -> String {
    format!("{{ error: \"{status}\" }}")
}

#[launch]
fn rocket() -> _ {
    match server_functions::setup_database(&get_connector()) {
        Ok(_) => {},
        Err(err) => panic!("Failed to setup database: {}", match err {
            ServerError::SqlError(str) => str,
            other => format!("{:?}", other)
        }),
    };
    rocket::build()
        .mount("/", routes![get_table_orders])
        .mount("/", routes![post_table_order])
        .mount("/", routes![get_table_order])
        .mount("/", routes![delete_table_order])
        .mount("/", routes![get_menu_items])
        .register("/", catchers![internal_error, not_found])
}

fn get_connector() -> database_connection::DefaultDatabaseConnector {
    database_connection::DefaultDatabaseConnector { path: DATABASE_PATH.to_string() }
}