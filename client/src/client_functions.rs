use reqwest::StatusCode;
use rocket::serde::{ Deserialize };
use rocket::serde::json::{ Json, to_string, from_str };
use server::rest_bodies::Orders;
use uuid::Uuid;

use server::{ rest_bodies, rest_responses };

use crate::web_connection::WebConnection;

pub fn get_all_orders(web_connection: &dyn WebConnection, host: String, table_number: u32) -> Result<Vec<rest_responses::Order>, String> {
    let web_response = web_connection.get(format!("{host}/tables/{table_number}/orders"))
        .map_err(|e| e.text)?;
    
    match web_response.status {
        StatusCode::OK => {
            let body = web_response.body
                .map_err(|e| e.to_string())?;
            Ok(from_str::<rest_responses::Orders>(&body)
                .map_err(|e| e.to_string())?
                .orders)
        },
        status => Result::Err(status.as_str().to_string())
    }
}

pub fn get_order(web_connection: &dyn WebConnection, host: String, table_number: u32, order_id: u32) -> Result<rest_responses::Order, String> {
    let web_response = web_connection.get(format!("{host}/tables/{table_number}/orders/{order_id}"))
        .map_err(|e| e.text)?;
    
    match web_response.status {
        StatusCode::OK => {
            let body = web_response.body
                .map_err(|e| e.to_string())?;
            from_str::<rest_responses::Order>(&body)
                .map_err(|e| e.to_string())
        },
        status => Result::Err(status.as_str().to_string())
    }
}

pub fn add_orders<F>(web_connection: &dyn WebConnection, host: String, table_number: u32, menu_item_ids: Vec<u32>, should_retry: F) -> Result<Vec<rest_responses::Order>, String>
                        where F: Fn() -> bool {
    let orders = rest_bodies::Orders {
        idempotency_key: Option::Some(Uuid::new_v4().to_string()),
        orders: menu_item_ids.iter().map(|i| rest_bodies::Order {
            menu_item_id: *i,
        }).collect()
    };

    let web_response = loop {
        let resp = web_connection.post(
            format!("{host}/tables/{table_number}/orders"),
            to_string(&orders).map_err(|e| e.to_string())?);
        let retry = resp
            .as_ref()
            .err()
            .map_or(false, |e|
                e.is_timeout && should_retry());
        if !retry {
            break resp.map_err(|e| e.text)?;
        }
    };

    match web_response.status {
        StatusCode::OK => {
            let body = web_response.body
                .map_err(|e| e.to_string())?;
            Ok(from_str::<rest_responses::Orders>(&body)
                .map_err(|e| e.to_string())?
                .orders)
        },
        status => Result::Err(status.to_string())
    }
}

pub fn delete_order(web_connection: &dyn WebConnection, host: String, table_number: u32, order_id: u32) -> Result<(), String> {
    let web_response = web_connection.delete(format!("{host}/tables/{table_number}/orders/{order_id}"))
        .map_err(|e| e.text)?;
    match web_response.status {
        StatusCode::NO_CONTENT => Ok(()),
        status => Result::Err(status.as_str().to_string())
    }
}

pub fn get_menu_items(web_connection: &dyn WebConnection, host: String) -> Result<rest_responses::MenuItems, String> {
    let web_response = web_connection.get(format!("{host}/menu-items"))
        .map_err(|e| e.text)?;

    match web_response.status {
        StatusCode::OK => {
            let body = web_response.body
                .map_err(|e| e.to_string())?;
            from_str::<rest_responses::MenuItems>(&body)
                .map_err(|e| e.to_string())
        },
        status => Result::Err(status.as_str().to_string())
    }
}