use rocket::{ catch, delete, get, post };
use rusqlite::Result;
use rocket::http::{ Status, ContentType };
use rocket::serde::json::{ Json, to_string };
use server_errors::ServerError;
use rocket::Request;
use rocket::State;

use crate::rest_bodies;
use crate::server_errors;
use crate::server_functions;
use crate::database_connection::DatabaseConnector;

#[get("/tables/<table_number>/orders")]
pub fn get_table_orders(table_number: u32, database_connector: &State<Box<dyn DatabaseConnector>>) -> (Status, (ContentType, String)) {
    match server_functions::get_orders(database_connector.inner().as_ref(), table_number) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(item_string) => (Status::Ok, (ContentType::JSON, item_string)),
            Result::Err(_) => (Status::InternalServerError, (ContentType::JSON, "{ \"error\": \"Server error. Failed to get data.\" }".to_string()))
        },
        Result::Err(_) => (Status::InternalServerError, (ContentType::JSON, "{ \"error\": \"Server error. Failed to get data.\" }".to_string()))
    }
}

#[post("/tables/<table_id>/orders", format = "json", data = "<orders_data>")]
pub fn post_table_order(table_id: u32, orders_data: Json<rest_bodies::Orders>, database_connector: &State<Box<dyn DatabaseConnector>>) -> (Status, (ContentType, String)) {
    match server_functions::add_orders(database_connector.inner().as_ref(), table_id, orders_data.into_inner()) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(item_string) => (Status::Ok, (ContentType::JSON, item_string)),
            Result::Err(_) => (Status::InternalServerError, (ContentType::JSON, "{ \"error\": \"Server error. Failed to add data.\" }".to_string()))
        },
        Result::Err(e) => match e {
            ServerError::Idempotency() => (Status::Conflict, (ContentType::JSON, "{ \"error\": \"This order has already been added.\" }".to_string())),
            ServerError::DataNotFound() => (Status::UnprocessableEntity, (ContentType::JSON, "{ error: \"The provided menu_item_id does not exist.\" }".to_string())),
            _ => (Status::InternalServerError, (ContentType::JSON, "{ \"error\": \"Server error. Failed to add data.\" }".to_string()))
        }
    }
}

#[get("/tables/<table_number>/orders/<order_id>")]
pub fn get_table_order(table_number: u32, order_id: u32, database_connector: &State<Box<dyn DatabaseConnector>>) -> (Status, (ContentType, String)) {
    match server_functions::get_order(database_connector.inner().as_ref(), table_number, order_id) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(item_string) => (Status::Ok, (ContentType::JSON, item_string)),
            Result::Err(_) => (Status::InternalServerError, (ContentType::JSON, "{ \"error\": \"Server error. Failed to get data.\" }".to_string()))
        },
        Result::Err(e) => match e {
            ServerError::DataNotFound() => (Status::NotFound, (ContentType::JSON, "{ \"error\": \"Provided order does not exist for provided table.\" }".to_string())),
            _ => (Status::InternalServerError, (ContentType::JSON, "{ \"error\": \"Server error. Failed to get data.\" }".to_string()))
        }
    }
}

#[delete("/tables/<table_number>/orders/<order_id>")]
pub fn delete_table_order(table_number: u32, order_id: u32, database_connector: &State<Box<dyn DatabaseConnector>>) -> (Status, (ContentType, String)) {
    match server_functions::delete_order(database_connector.inner().as_ref(), table_number, order_id) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(_) => (Status::NoContent, (ContentType::JSON, "{}".to_string())),
            Result::Err(_) => (Status::InternalServerError, (ContentType::JSON, "{ \"error\": \"Server error. Failed to delete data.\" }".to_string()))
        },
        Result::Err(_) => (Status::InternalServerError, (ContentType::JSON, "{ \"error\": \"Server error. Failed to delete data.\" }".to_string()))
    }
}

#[get("/menu-items")]
pub fn get_menu_items(database_connector: &State<Box<dyn DatabaseConnector>>) -> (Status, (ContentType, String)) {
    match server_functions::get_menu_items(database_connector.inner().as_ref()) {
        Result::Ok(items) => match to_string(&items) {
            Result::Ok(item_string) => (Status::Ok, (ContentType::JSON, item_string)),
            Result::Err(_) => (Status::InternalServerError, (ContentType::JSON, "{ \"error\": \"Server error. Failed to get data.\" }".to_string()))
        },
        Result::Err(_) => (Status::InternalServerError, (ContentType::JSON, "{ \"error\": \"Server error. Failed to get data.\" }".to_string()))
    }
}

#[catch(400)]
pub fn internal_error() -> &'static str {
    "{ \"error\": \"Request format could not be understood\" }"
}

#[catch(404)]
pub fn not_found(_: &Request) -> &'static str {
    "{ \"error\": \"Resource could not be found\" }"
}

#[catch(422)]
pub fn unprocessable_entity(_: &Request) -> &'static str {
    "{ \"error\": \"Bad format\" }"
}

#[catch(default)]
pub fn default(status: Status, _: &Request) -> String {
    format!("{{ \"error\": \"{status}\" }}")
}