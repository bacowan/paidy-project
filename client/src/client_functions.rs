use reqwest::StatusCode;
use rocket::serde::{ Deserialize };
use rocket::serde::json::{ Json, to_string, from_str };
use uuid::Uuid;

use server::{ rest_bodies, rest_responses };


pub fn get_all_orders(table_id: String) -> Result<Vec<rest_responses::Order>, String> {
    return Result::Ok(Vec::new());
}

pub fn get_order(table_id: String, order_id: String) -> Result<rest_responses::Order, String> {
    return Result::Ok(rest_responses::Order {
        id: 0,
        menu_item_id: 0,
        minutes_to_cook: 1,
        menu_item_name: "Pizza".to_string()
    });
}

pub async fn add_orders<F>(host: String, table_number: String, menu_item_ids: Vec<u32>, should_retry: F) -> Result<Vec<u32>, String>
                        where F: Fn() -> bool {
    let orders = rest_bodies::Orders {
        orders: menu_item_ids.iter().map(|i| rest_bodies::Order {
            menu_item_id: *i,
            idempotency_key: Uuid::new_v4().to_string()
        }).collect()
    };

    let client = reqwest::Client::new();

    let web_response = loop {
        let resp = client.post(format!("{host}/orders/{table_number}"))
            .body(to_string(&orders).map_err(|e| e.to_string())?)
            .send()
            .await;
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
                .await
                .map_err(|e| e.to_string())?;
            from_str::<Vec<u32>>(&body)
                .map_err(|e| e.to_string())
        },
        status => Result::Err(status.to_string())
    }
}

pub fn delete_order(table_id: String, order_id: String) -> Result<(), String> {
    return Result::Ok(());
}

pub async fn get_menu_items(host: String) -> Result<rest_responses::MenuItems, String> {
    let web_response = reqwest::get(format!("{host}/menu-items"))
        .await.map_err(|e| e.to_string())?;

    match web_response.status() {
        StatusCode::OK => {
            let body = web_response.text()
                .await.map_err(|e| e.to_string())?;
            from_str::<rest_responses::MenuItems>(&body)
                .map_err(|e| e.to_string())
        },
        status => Result::Err(status.as_str().to_string())
    }
}