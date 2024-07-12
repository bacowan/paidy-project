use std::borrow::Borrow;
use std::future::Future;
use futures::executor::block_on;
use rand::rngs::{StdRng, ThreadRng};
use std::io::{self, Write};
use server::rest_bodies;
use server::rest_responses::{self, MenuItem};
use std::thread::{self, Thread};
use std::time::Duration;
use rand::{Rng, SeedableRng};
use rand::seq::SliceRandom;
use crate::web_connection::DefaultWebConnection;
use crate::client_function_interface::{ClientFunctionInterface, DefaultClientFunctionInterface};

const HOST: &str = "http://127.0.0.1:8000";
const TABLE_COUNT: u32 = 5;
const MIN_DELAY_MILLIS: u64 = 300;
const MAX_DELAY_MILLIS: u64 = 4000;

pub struct TableOrderPair {
    pub table_id: u32,
    pub order_id: u32
}

pub struct SimInjectionParams<T>
        where T: ClientFunctionInterface {
    pub client_functions: T,
    pub rng: StdRng
}

pub fn client_tablet(client_number: u32) {
    let mut injection = SimInjectionParams {
        client_functions: DefaultClientFunctionInterface{},
        rng: SeedableRng::from_entropy()
    };

    let mut added_items: Vec<TableOrderPair> = Vec::new();
    println!("{}", add_random_order(&mut injection, client_number, &mut added_items));
    loop {
        thread::sleep(Duration::from_millis(injection.rng.gen_range(MIN_DELAY_MILLIS..MAX_DELAY_MILLIS)));
        let to_print = match injection.rng.gen_range(1..5) {
            1 => add_random_order(&mut injection, client_number, &mut added_items),
            2 => delete_random_order(&mut injection, client_number, &mut added_items),
            3 => query_random_table(&mut injection, client_number),
            _ => query_random_table_item(&mut injection, client_number, &added_items)
        };
        println!("{}", to_print);
    }
}

pub fn add_random_order<T>(params: &mut SimInjectionParams<T>, client_number: u32, added_items: &mut Vec<TableOrderPair>) -> String
        where T: ClientFunctionInterface {
    let table_number = params.rng.gen_range(1..TABLE_COUNT + 1);
    let menu_item_names = (0..params.rng.gen_range(1..3))
        .map(|_| match params.rng.gen_range(1..6) {
            1 => "Hamburger".to_string(),
            2 => "Salad".to_string(),
            3 => "Sushi".to_string(),
            4 => "Ice Cream".to_string(),
            _ => "Soda".to_string()
        })
        .collect::<Vec<String>>();
    
    let result = add_to_table(params, table_number, menu_item_names);

    match result {
        Ok(orders) => {
            added_items.extend(orders.iter().map(|o| TableOrderPair {
                table_id: table_number,
                order_id: o.id
            }));
            format!("Client {} added {} to table {}",
                client_number,
                orders.iter().map(|o| o.menu_item_name.to_string()).collect::<Vec<String>>().join(", "),
                table_number)
        },
        Err(e) => format!("Client {} encountered an error trying to add orders to table {}: {}",
            client_number,
            table_number,
            e.to_string())
    }
}

fn add_to_table<T>(params: &SimInjectionParams<T>, table_number: u32, menu_item_names: Vec<String>) -> Result<Vec<rest_responses::Order>, String>
        where T: ClientFunctionInterface {
    let connection = DefaultWebConnection {};
    let menu_items = params.client_functions.get_menu_items(&connection, HOST.to_string())?;
    let item_ids = menu_item_names
        .iter()
        .map(|n| menu_items.menu_items.iter().find(|m| m.name == n.to_string()))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap().id)
        .collect::<Vec<u32>>();
    params.client_functions.add_orders(&connection, HOST.to_string(), table_number, item_ids, || false)
}

pub fn delete_random_order<T>(params: &mut SimInjectionParams<T>, client_number: u32, added_items: &mut Vec<TableOrderPair>) -> String
        where T: ClientFunctionInterface {
    let connection = DefaultWebConnection {};
    match added_items.choose(&mut params.rng) {
        Some(item_to_delete) => {
            match params.client_functions.delete_order(&connection, HOST.to_string(), item_to_delete.table_id, item_to_delete.order_id) {
                Ok(()) => {
                    let ret = format!("Client {} deleted order {} from table {}.",
                        client_number,
                        item_to_delete.order_id,
                        item_to_delete.table_id);
                        
                    if let Some(item_index) = added_items.iter().position(|i|
                        i.table_id == item_to_delete.table_id && i.order_id == item_to_delete.order_id) {
                            added_items.remove(item_index);
                    }
                    ret
                },
                Err(e) => format!("Client {} tried to delete order {} from table {}, but encountered an error: {}.",
                    client_number,
                    item_to_delete.order_id,
                    item_to_delete.table_id,
                    e)
            }
        },
        None => format!("Client {} tried to delete a random order but had none to delete.", client_number)
    }
}

pub fn query_random_table<T>(params: &mut SimInjectionParams<T>, client_number: u32) -> String
        where T: ClientFunctionInterface {
    let connection = DefaultWebConnection {};
    let table_number = params.rng.gen_range(1..TABLE_COUNT + 1);
    match params.client_functions.get_all_orders(&connection, HOST.to_string(), table_number) {
        Ok(orders) => {
            format!("Client {} queried orders for table {}, which had {} orders, including {} for {} minutes.",
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
        Err(e) => format!("Client {} encountered an error trying to query orders for table {}: {}",
            client_number,
            table_number,
            e.to_string())
    }
}

pub fn query_random_table_item<T>(params: &mut SimInjectionParams<T>, client_number: u32, added_items: &Vec<TableOrderPair>) -> String
        where T: ClientFunctionInterface {
    let connection = DefaultWebConnection {};
    if let Some(item_to_query) = added_items.choose(&mut params.rng) {
        match params.client_functions.get_order(&connection, HOST.to_string(), item_to_query.table_id, item_to_query.order_id) {
            Ok(order) => format!(
                "Client {} queried order with ID {} for table {}: order id {}, {}, {} minutes",
                client_number,
                item_to_query.table_id,
                item_to_query.order_id,
                order.id,
                order.menu_item_name,
                order.minutes_to_cook),
            Err(e) => format!("Client {} encountered an error trying to query a random table order: {}",
                client_number,
                e.to_string())
        }
    }
    else {
        format!("Client {} tried to query a random order but had none to query.", client_number)
    }
}