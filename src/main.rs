use std::io::{self, Write};

use client::MenuItem;
mod client;

fn main() {
    match client::get_menu_items() {
        Result::Ok(items) => main_loop(items),
        Result::Err(e) => println!("{e}")
    }
}

fn main_loop(menu_items: Vec<MenuItem>) {
    loop {
        println!("Type one of the following commands to continue:");
        println!("  1: List orders");
        println!("  2: Add orders");
        println!("  3: Delete order");
        println!("  exit: Exit the application");

        let command = get_user_input();

        match command.trim() {
            "1" => list_orders(),
            "2" => add_orders(&menu_items),
            "3" => delete_order(),
            "exit" => return,
            _ => println!("Invalid command")
        }
    }
}

fn list_orders() {
    println!("Input a table number, or type \"exit\" to go back.");
    loop {
        let table_number = get_user_input();
        match table_number.trim() {
            "" => {},
            "exit" => return,
            n => {
                list_orders_for_table(n.to_string());
                return;
            }
        }
    }
}

fn list_orders_for_table(table_number: String) {
    println!("Input an order id or no text to show all orders. Type \"exit\" to go back.");
    let order_id = get_user_input();

    let orders_result = match order_id.trim() {
        "exit" => return,
        "" => list_orders_multiple(table_number),
        n => list_orders_single(table_number, n.to_string())
    };

    let order_text = match orders_result {
        Result::Ok(orders) =>
            if orders.len() == 0 {
                "No orders for this table".to_string()
            }
            else {
                orders
                    .iter()
                    .map(|o| format!("{id}: {minutes}", id = o.id, minutes = o.minutes_to_cook))
                    .collect::<Vec<String>>()
                    .join("\r\n")
            },
        Result::Err(e) => e
    };

    println!("{order_text}");
    println!("Press enter to continue.");
    io::stdin().read_line(&mut String::new()).expect("Failed to read line");
}

fn list_orders_multiple(table_id: String) -> Result<Vec<client::Order>, String> {
    return client::get_all_orders(table_id);
}

fn list_orders_single(table_id: String, order_id: String) -> Result<Vec<client::Order>, String> {
    match client::get_order(table_id, order_id) {
        Result::Ok(order) => Result::Ok(vec![order]),
        Result::Err(err) => Result::Err(err.to_string())
    }
}

fn add_orders(menu_items: &Vec<MenuItem>) {
    println!("Input a table number, or type \"exit\" to go back.");
    loop {
        let table_number = get_user_input();
    
        match table_number.trim() {
            "" => {},
            "exit" => return,
            n => {
                add_orders_for_table(n.to_string(), &menu_items);
                return;
            }
        }
    }
}

fn add_orders_for_table(table_number: String, menu_items: &Vec<MenuItem>) {
    println!("Input a menu item ID to add an order item. Input no text to confirm. Input \"exit\" to cancel and go back.");
    println!("Menu item IDs are as follows:");

    let menu_item_text = menu_items
        .iter()
        .map(|i| format!("{id}: {name}", id = i.id, name = i.name))
        .collect::<Vec<String>>()
        .join("\r\n");

    println!("{menu_item_text}");

    let mut staged_items: Vec<String> = Vec::new();
    loop {
        let menu_item_id = get_user_input();

        match menu_item_id.trim() {
            "" => break,
            id => staged_items.push(id.to_string())
        }
    }

    if staged_items.len() == 0 {
        println!("No items added.");
    }
    else {
        let result = client::add_orders(table_number, staged_items);
        let string_result = match result {
            Result::Ok(ids) => format!("Orders successfully added. IDs for added orders are: {}", ids.join("; ")),
            Result::Err(e) => e
        };
    
        println!("{string_result}");
    }

    println!("Press enter to continue.");
    io::stdin().read_line(&mut String::new()).expect("Failed to read line");
}

fn delete_order() {
    println!("Input a table number, or type \"exit\" to go back.");
    
    loop {
        let table_number = get_user_input();

        match table_number.trim() {
            "" => {},
            "exit" => return,
            id => {
                delete_order_for_table(id.to_string());
                return;
            }
        }
    }
}

fn delete_order_for_table(table_number: String) {
    println!("Input an order number, or type \"exit\" to go back.");

    loop {
        let order_number = get_user_input();

        match order_number.trim() {
            "" => {},
            "exit" => return,
            id => {
                let message = match client::delete_order(table_number, id.to_string()) {
                    Result::Ok(()) => "Order deleted successfully".to_string(),
                    Result::Err(e) => e
                };
                println!("{message}");
                println!("Press enter to continue.");
                io::stdin().read_line(&mut String::new()).expect("Failed to read line");
                return;
            }
        }
    }
}

fn get_user_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}