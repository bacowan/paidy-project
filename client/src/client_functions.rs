use reqwest::StatusCode;
use rocket::serde::{ Deserialize };
use rocket::serde::json::{ Json, to_string, from_str };
use server::rest_bodies::Orders;
use uuid::Uuid;

use server::{ rest_bodies, rest_responses };


pub fn get_all_orders(host: String, table_number: u32) -> Result<Vec<rest_responses::Order>, String> {
    let web_response = reqwest::blocking::get(format!("{host}/tables/{table_number}/orders"))
        .map_err(|e| e.to_string())?;

    match web_response.status() {
        StatusCode::OK => {
            let body = web_response.text()
                .map_err(|e| e.to_string())?;
            Ok(from_str::<rest_responses::Orders>(&body)
                .map_err(|e| e.to_string())?
                .orders)
        },
        status => Result::Err(status.as_str().to_string())
    }
}

pub fn get_order(host: String, table_number: u32, order_id: u32) -> Result<rest_responses::Order, String> {
    let web_response = reqwest::blocking::get(format!("{host}/orders/{order_id}"))
        .map_err(|e| e.to_string())?;

    match web_response.status() {
        StatusCode::OK => {
            let body = web_response.text()
                .map_err(|e| e.to_string())?;
            from_str::<rest_responses::Order>(&body)
                .map_err(|e| e.to_string())
        },
        status => Result::Err(status.as_str().to_string())
    }
}

pub fn add_orders<F>(host: String, table_number: u32, menu_item_ids: Vec<u32>, should_retry: F) -> Result<Vec<rest_responses::Order>, String>
                        where F: Fn() -> bool {
    let orders = rest_bodies::Orders {
        orders: menu_item_ids.iter().map(|i| rest_bodies::Order {
            menu_item_id: *i,
            idempotency_key: Uuid::new_v4().to_string()
        }).collect()
    };

    let client = reqwest::blocking::Client::new();

    let web_response = loop {
        let resp = client.post(format!("{host}/tables/{table_number}/orders"))
            .header("Content-Type", "application/json")
            .body(to_string(&orders).map_err(|e| e.to_string())?)
            .send();
        let retry = resp
            .as_ref()
            .err()
            .map_or(false, |e| e.is_timeout() && should_retry());
        if !retry {
            break resp.map_err(|e| e.to_string())?;
        }
    };

    match web_response.status() {
        StatusCode::OK => {
            let body = web_response.text()
                .map_err(|e| e.to_string())?;
            Ok(from_str::<rest_responses::Orders>(&body)
                .map_err(|e| e.to_string())?
                .orders)
        },
        status => Result::Err(status.to_string())
    }
}

pub fn delete_order(table_number: u32, order_id: u32) -> Result<(), String> {
    return Result::Ok(());
}

pub fn get_menu_items(host: String) -> Result<rest_responses::MenuItems, String> {
    let web_response = reqwest::blocking::get(format!("{host}/menu-items"))
        .map_err(|e| e.to_string())?;

    match web_response.status() {
        StatusCode::OK => {
            let body = web_response.text()
                .map_err(|e| e.to_string())?;
            from_str::<rest_responses::MenuItems>(&body)
                .map_err(|e| e.to_string())
        },
        status => Result::Err(status.as_str().to_string())
    }
}