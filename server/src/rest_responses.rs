use rocket::serde::{ Deserialize, Serialize };

#[derive(Deserialize)]
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MenuItem {
    pub id: u32,
    pub name: String
}

#[derive(Deserialize)]
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MenuItems {
    pub menu_items: Vec<MenuItem>
}

#[derive(Deserialize)]
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Orders {
    pub orders: Vec<Order>
}

#[derive(Deserialize)]
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Order {
    pub id: u32,
    pub menu_item_id: u32,
    pub menu_item_name: String,
    pub minutes_to_cook: u32
}