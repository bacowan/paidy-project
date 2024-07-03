use std::io::{self, Write};
mod client;

fn main() {
    println!("Welcome to the menu application. Type a command, or h for a list of commands.");
    loop {
        let mut command = String::new();

        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command_parts: Vec<String> = command
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        

        match command_parts.split_first() {
            Some((command, params)) => {
                match command.as_str() {
                    "menu" => show_menu(),
                    "list-orders" => list_orders(params),
                    "add-orders" => add_orders(params),
                    "delete-order" => delete_order(params),
                    "exit" => return,
                    _ => show_help()
                }
            },
            _ => continue
        }
    }
}

fn show_help() {
    println!("Commands:");
    println!("  h: display this help menu");
    println!("  menu: display all available menu items");
    println!("  list-orders <table-id>: display all current orders for the table with ID table-id");
    println!("  list-orders <table-id> <order-id>: display the order with ID order-id for the table with the ID table-id");
    println!("  add-orders <table-id> [<menu-item-id> ...]: adds the menu items with IDs menu-item-id to the table, and displays each of the order ids assigned to them");
    println!("  delete-order <table-id> <order-id>: remove an order with ID order-id for the table with the ID table-id");
    println!("  exit: exit the application");
}

fn list_orders(params: &[String]) {
    let order_result = match params {
        [table_id, order_id] => list_orders_single(table_id.to_string(), order_id.to_string()),
        [table_id] => list_orders_multiple(table_id.to_string()),
        _ => {
            let ret: Result<Vec<client::Order>, String> = Result::Err("Invalid parameters for command".to_string());
            ret
        }
    };

    match order_result {
        Result::Ok(orders) => {
            let ret = orders 
                .into_iter()
                .map(|order| format_order_string(order))
                .collect::<Vec<String>>()
                .join("\r\n");
            println!("[\r\n{ret}\r\n]");
        },
        Result::Err(err) => println!("{err}")
    };
}

fn format_order_string(order: client::Order) -> String {
    let mut ret = String::new();
    ret.push_str("\t{\r\n");
    ret.push_str(format!("\t\tid: {id},\r\n", id = order.id).as_str());
    ret.push_str(format!("\t\tmenu-item-id: {id},\r\n", id = order.menu_item_id).as_str());
    ret.push_str(format!("\t\tminutes-to-cook: {min}\r\n", min = order.minutes_to_cook).as_str());
    ret.push_str("\t},");
    return ret;
}

fn list_orders_multiple(table_id: String) -> Result<Vec<client::Order>, String> {
    return client::get_all_orders(table_id);
}

fn list_orders_single(table_id: String, order_id: String) -> Result<Vec<client::Order>, String> {
    match client::get_order(table_id, order_id) {
        Result::Ok(order) => {
            let ret: Result<Vec<client::Order>, String> = Result::Ok(vec![order]);
            ret
        }
        Result::Err(err) => {
            let ret: Result<Vec<client::Order>, String> = Result::Err(err.to_string());
            ret
        }
    }
}

fn add_orders(params: &[String]) {
    let menu_items = match params {
        [table_id, item_ids @ ..] if item_ids.len() > 0 =>
            client::add_orders(table_id.to_string(), item_ids.to_vec()),
        _ => {
            let ret: Result<Vec<String>, String> = Result::Err("Invalid command parameters".to_string());
            ret
        }
    };
    
    match menu_items {
        Result::Ok(items) => println!("{ids}", ids = items.join("; ")),
        Result::Err(err) => println!("{err}")
    }
}

fn delete_order(params: &[String]) {
    let menu_items = match params {
        [table_id, item_id] =>
            client::delete_order(table_id.to_string(), item_id.to_string()),
        _ => {
            let ret: Result<(), String> = Result::Err("Invalid command parameters".to_string());
            ret
        }
    };
    
    match menu_items {
        Result::Ok(()) => println!("Successfully deleted"),
        Result::Err(err) => println!("{err}")
    }
}

fn show_menu() {
    let menu_items = client::get_menu_items();
    
    match menu_items {
        Result::Ok(items) => {
            let ret = items 
                .into_iter()
                .map(|item| format_menu_item(item))
                .collect::<Vec<String>>()
                .join("\r\n");
            println!("[\r\n{ret}\r\n]");
        },
        Result::Err(err) => println!("{err}")
    }
}

fn format_menu_item(menu_item: client::MenuItem) -> String {
    let mut ret = String::new();
    ret.push_str("\t{\r\n");
    ret.push_str(format!("\t\tid: {id},\r\n", id = menu_item.id).as_str());
    ret.push_str(format!("\t\tname: {name},\r\n", name = menu_item.name).as_str());
    ret.push_str("\t},");
    return ret;
}