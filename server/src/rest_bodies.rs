use rocket::serde::{ Serialize, Deserialize };

#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Orders {
    pub orders: Vec<Order>
}

#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Order {
    pub menu_item_id: u32,
    pub idempotency_key: String
}