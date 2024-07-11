use std::env;

use client::web_connection::DefaultWebConnection;
use client::client_functions;

mod sim;

const HOST: &str = "http://127.0.0.1:8000";


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 => sim::run_sim(),
        _ => {
            match args.iter().map(|a| a.as_str()).collect::<Vec<&str>>().as_slice() {
                ["show-menu-items"] => Ok(menu_items_command()),
                ["add-orders", table_number, menu_item_ids @ ..] => Ok(add_orders(table_number, menu_item_ids.to_vec()))
            }
        }
    }
}

fn menu_items_command() {
    let connection = DefaultWebConnection {};
    let prnt = match client_functions::get_menu_items(&connection, HOST.to_string()) {
        Ok(menu_items) => menu_items.menu_items
            .iter()
            .map(|i| format!("{}: {}", i.id, i.name))
                .collect::<Vec<String>>()
                .join("\r\n"),
        Err(e) => format!("Encountered an Error: {}", e)
    };
    println!("{prnt}");
}

fn add_orders(table_number_string: String, menu_item_string_ids: Vec<String>) {
    let connection = DefaultWebConnection {};

    let prnt = match table_number_string.parse::<u32>() {
        Ok(table_number) => {
            match menu_item_string_ids.iter().map(|i| i.parse::<u32>()).collect() {
                Ok(menu_item_ids) => client_functions::add_orders(&connection, HOST.to_string(), table_number, menu_item_ids, should_retry),
                Err(x) => format!("Integer values are required for")
            }
                
        }
    }
    client_functions::add_orders(&connection, HOST.to_string(), table_number, menu_item_ids, should_retry);
}