use std::io::{self, Write};
use tokio;
use server::rest_bodies;
use server::rest_responses;

mod client_functions;

const HOST: &str = "http://127.0.0.1:8000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match client_functions::get_menu_items(HOST.to_string()).await {
        Result::Ok(items) => main_loop(items.menu_items).await,
        Result::Err(e) => println!("Failed retrieving menu items: {e}")
    };

    Result::Ok(())
}

async fn main_loop(menu_items: Vec<rest_responses::MenuItem>) {
    loop {
        println!("Type one of the following commands to continue:");
        println!("  1: List orders");
        println!("  2: Add orders");
        println!("  3: Delete order");
        println!("  exit: Exit the application");

        let command = get_user_input();

        match command.trim() {
            "1" => list_orders(),
            "2" => add_orders(&menu_items).await,
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

async fn list_orders_for_table(table_number: String) {
    println!("Input an order id or no text to show all orders. Type \"exit\" to go back.");
    let order_id = get_user_input();

    let orders_result = match order_id.trim() {
        "exit" => return,
        "" => list_orders_multiple(table_number).await,
        n => list_orders_single(n.to_string()).await
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

async fn list_orders_multiple(table_number: String) -> Result<Vec<rest_responses::Order>, String> {
    return client_functions::get_all_orders(table_number).await;
}

async fn list_orders_single(order_id: String) -> Result<Vec<rest_responses::Order>, String> {
    match client_functions::get_order(order_id).await {
        Result::Ok(order) => Result::Ok(vec![order]),
        Result::Err(err) => Result::Err(err.to_string())
    }
}

async fn add_orders(menu_items: &Vec<rest_responses::MenuItem>) {
    println!("Input a table number, or type \"exit\" to go back.");
    loop {
        let table_number = get_user_input();
    
        match table_number.trim() {
            "" => {},
            "exit" => return,
            n => {
                add_orders_for_table(n.to_string(), &menu_items).await;
                return;
            }
        }
    }
}

async fn add_orders_for_table(table_number: String, menu_items: &Vec<rest_responses::MenuItem>) {
    println!("Input a menu item ID to add an order item. Input no text to confirm. Input \"exit\" to cancel and go back.");
    println!("Menu item IDs are as follows:");

    let menu_item_text = menu_items
        .iter()
        .map(|i| format!("{id}: {name}", id = i.id, name = i.name))
        .collect::<Vec<String>>()
        .join("\r\n");

    println!("{menu_item_text}");

    let mut staged_items: Vec<u32> = Vec::new();
    loop {
        let menu_item_id = match get_user_input().trim() {
            "" => break,
            "exit" => return,
            id => match id.parse::<u32>() {
                Ok(id_int) => id_int,
                Err(_) => continue
            }
        };
        staged_items.push(menu_item_id);
    }

    if staged_items.len() == 0 {
        println!("No items added.");
    }
    else {
        let result = client_functions::add_orders(
            HOST.to_string(),
            table_number,
            staged_items,
            should_retry).await;
        let string_result = match result {
            Result::Ok(()) => "Orders successfully added.".to_string(),
            Result::Err(e) => e.to_string()
        };
    
        println!("{string_result}");
    }

    println!("Press enter to continue.");
    io::stdin().read_line(&mut String::new()).expect("Failed to read line");
}

fn should_retry() -> bool {
    println!("The server timed out. Would you like to retry? If not, make sure you check if the item was added afterwords.");
    println!("Y: retry; N: exit");
    loop {
        break match get_user_input().trim() {
            "Y" => true,
            "N" => false,
            _ => continue
        }
    }
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
                let message = match client_functions::delete_order(table_number, id.to_string()) {
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