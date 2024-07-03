pub struct Order {
    pub id: String,
    pub menu_item_id: String,
    pub minutes_to_cook: u8
}

pub struct MenuItem {
    pub id: String,
    pub name: String
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

pub fn get_menu_items() -> Result<Vec<MenuItem>, String> {
    return Result::Ok(vec![
        MenuItem {
            id: "1".to_string(),
            name: "Burger".to_string()
        },
        MenuItem {
            id: "2".to_string(),
            name: "Salad".to_string()
        }
    ]);
}