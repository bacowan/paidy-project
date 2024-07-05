use reqwest::StatusCode;
use rocket::serde::{ Deserialize };
use rocket::serde::json::{ Json, to_string, from_str };

pub struct Order {
    pub id: String,
    pub menu_item_id: String,
    pub minutes_to_cook: u8
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MenuItem {
    pub id: u32,
    pub name: String
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MenuItems {
    pub menu_items: Vec<MenuItem>
}

pub fn get_all_orders(table_id: String) -> Result<Vec<Order>, String> {
    return Result::Ok(Vec::new());
}

pub fn get_order(table_id: String, order_id: String) -> Result<Order, String> {
    return Result::Ok(Order {
        id: "test".to_string(),
        menu_item_id: "test".to_string(),
        minutes_to_cook: 1
    });
}

pub fn add_orders(table_id: String, menu_item_ids: Vec<String>) -> Result<Vec<String>, String> {
    return Result::Ok(vec!["1".to_string(), "2".to_string()]);
}

pub fn delete_order(table_id: String, order_id: String) -> Result<(), String> {
    return Result::Ok(());
}

pub async fn get_menu_items(endpoint: String) -> Result<MenuItems, String> {
    let web_response = reqwest::get(format!("{endpoint}/menu-items"))
        .await.map_err(|e| e.to_string())?;

    match web_response.status() {
        StatusCode::OK => {
            let body = web_response.text()
                .await.map_err(|e| e.to_string())?;
            from_str::<MenuItems>(&body)
                .map_err(|e| e.to_string())
        },
        status => Result::Err(status.as_str().to_string())
    }
}