use std::future::Future;
use futures::executor::block_on;
use std::io::{self, Write};
use server::rest_bodies;
use server::rest_responses::{self, MenuItem};
use std::thread;
use std::time::Duration;
use rand::Rng;
use rand::seq::SliceRandom;

mod client_functions;

const HOST: &str = "http://127.0.0.1:8000";
const TABLE_COUNT: u32 = 5;
const TABLET_COUNT: u32 = 10;
const RUN_TIME_MILLIS: u64 = 60000; // 1 minute

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for i in 1..TABLET_COUNT + 1 {
        thread::spawn(move || client_tablet(i));
    }

    thread::sleep(Duration::from_millis(RUN_TIME_MILLIS));
    Result::Ok(())
}

fn client_tablet(client_number: u32) {
    let mut added_items: Vec<(u32,u32)> = Vec::new();
    add_random_order(client_number, &mut added_items);
    loop {
        thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(300..4000)));
        match rand::thread_rng().gen_range(1..5) {
            1 => add_random_order(client_number, &mut added_items),
            2 => delete_random_order(client_number, &mut added_items),
            3 => query_random_table(client_number),
            _ => query_random_table_item(client_number, &added_items)
        };
    }
}

fn add_random_order(client_number: u32, added_items: &mut Vec<(u32,u32)>) {
    let table_number = rand::thread_rng().gen_range(1..TABLE_COUNT + 1);
    let menu_item_names = (0..rand::thread_rng().gen_range(1..3))
        .map(|_| match rand::thread_rng().gen_range(1..6) {
            1 => "Hamburger".to_string(),
            2 => "Salad".to_string(),
            3 => "Sushi".to_string(),
            4 => "Ice Cream".to_string(),
            _ => "Soda".to_string()
        })
        .collect::<Vec<String>>();
    
    let result = add_to_table(table_number, menu_item_names);

    match result {
        Ok(orders) => {
            added_items.extend(orders.iter().map(|o| (table_number, o.id)));
            println!("Client {} added {} to table {}",
                client_number,
                orders.iter().map(|o| o.menu_item_name.to_string()).collect::<Vec<String>>().join(", "),
                table_number);
        },
        Err(e) => println!("Client {} encountered an error trying to add orders to table {}: {}",
            client_number,
            table_number,
            e.to_string())
    };
}

fn add_to_table(table_number: u32, menu_item_names: Vec<String>) -> Result<Vec<rest_responses::Order>, String> {
    let menu_items = client_functions::get_menu_items(HOST.to_string())?;
    let item_ids = menu_item_names
        .iter()
        .map(|n| menu_items.menu_items.iter().find(|m| m.name == n.to_string()))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap().id)
        .collect::<Vec<u32>>();
    client_functions::add_orders(HOST.to_string(), table_number, item_ids, || false)
}

fn delete_random_order(client_number: u32, added_items: &mut Vec<(u32,u32)>) {
    let table_number = rand::thread_rng().gen_range(1..TABLE_COUNT + 1);
    let result = match client_functions::get_all_orders(HOST.to_string(), table_number) {
        Ok(orders) => delete_random_order_from_table(table_number, orders),
        Err(e) => Err(e)
    };

    match result {
        Ok(order_id) => {
            if let Some(index) = added_items.iter().position(|i| i.0 == table_number && i.1 == order_id) {
                added_items.swap_remove(index);
            }
            println!("Client {} deleted order {} from table {}.",
                client_number,
                order_id,
                table_number);
        },
        Err(e) => println!("Client {} tried to delete a random order from table {}, but encountered an error: {}.",
            client_number,
            table_number,
            e)
    };
}

fn delete_random_order_from_table(table_number: u32, orders: Vec<rest_responses::Order>) -> Result<u32, String> {
    match orders.choose(&mut rand::thread_rng()) {
        Some(order) => {
            client_functions::delete_order(table_number, order.id)?;
            Ok(order.id)
        },
        _ => Err("The table had no orders".to_string())
    }
}

fn query_random_table(client_number: u32) {
    let table_number = rand::thread_rng().gen_range(1..TABLE_COUNT + 1);
    match client_functions::get_all_orders(HOST.to_string(), table_number) {
        Ok(orders) => {

            println!("Client {} queried orders for table {}, which had {} orders, including {} for {} minutes.",
                client_number,
                table_number,
                orders.len(),
                match orders.first() {
                    Some(order) => order.menu_item_name.to_string(),
                    _ => "N/A".to_string()
                },
                match orders.first() {
                    Some(order) => order.minutes_to_cook.to_string(),
                    _ => "N/A".to_string()
                })
        },
        Err(e) => println!("Client {} encountered an error trying to query orders for table {}: {}",
            client_number,
            table_number,
            e.to_string())
    };
}

fn query_random_table_item(client_number: u32, added_items: &Vec<(u32,u32)>) {
    if let Some(item_to_query) = added_items.choose(&mut rand::thread_rng()) {
        match client_functions::get_order(HOST.to_string(), item_to_query.0, item_to_query.1) {
            Ok(order) => println!(
                "Client {} queried order with ID {} for table {}: order id {}, {}, {} minutes",
                client_number,
                item_to_query.1,
                item_to_query.0,
                order.id,
                order.menu_item_name,
                order.minutes_to_cook),
            Err(e) => println!("Client {} encountered an error trying to query a random table order: {}",
                client_number,
                e.to_string())
        }
    }
    else {
        println!("Client {} tried to query a random order but had none to query.", client_number);
    }
}