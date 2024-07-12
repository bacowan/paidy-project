use std::cell::RefCell;

use client::{client_function_interface::ClientFunctionInterface, web_connection::WebConnection};
use server::rest_responses;

pub struct MockClientFunctionInterface {
    pub should_fail: bool,
    pub was_delete_order_called: RefCell<bool>,
    pub was_add_orders_called: RefCell<bool>
}

pub const DEFAULT_RETURN_ORDER_ID: u32 = 5;
pub const DEFAULT_RETURN_ORDER_MENU_ID: u32 = 10;
pub const DEFAULT_RETURN_ORDER_MENU_NAME: &str = "Food";
pub const DEFAULT_RETURN_ORDER_MINUTES_TO_COOK: u32 = 20;


pub fn new() -> MockClientFunctionInterface {
    MockClientFunctionInterface {
        should_fail: false,
        was_add_orders_called: RefCell::new(false),
        was_delete_order_called: RefCell::new(false)
    }
}

fn new_default_return() -> rest_responses::Order {
    rest_responses::Order {
        id: DEFAULT_RETURN_ORDER_ID,
        menu_item_id: DEFAULT_RETURN_ORDER_MENU_ID,
        menu_item_name: DEFAULT_RETURN_ORDER_MENU_NAME.to_string(),
        minutes_to_cook: DEFAULT_RETURN_ORDER_MINUTES_TO_COOK
    }
}

impl ClientFunctionInterface for MockClientFunctionInterface {
    fn get_all_orders(&self, web_connection: &dyn WebConnection, host: String, table_number: u32) -> Result<Vec<rest_responses::Order>, String> {
        match self.should_fail {
            true => Err("".to_string()),
            false => Ok(vec![new_default_return()])
        }
    }

    fn get_order(&self, web_connection: &dyn WebConnection, host: String, table_number: u32, order_id: u32) -> Result<rest_responses::Order, String> {
        match self.should_fail {
            true => Err("".to_string()),
            false => Ok(new_default_return())
        }
    }

    fn add_orders<F>(&self, web_connection: &dyn WebConnection, host: String, table_number: u32, menu_item_ids: Vec<u32>, should_retry: F) -> Result<Vec<rest_responses::Order>, String>
            where F: Fn() -> bool {
        *self.was_add_orders_called.borrow_mut() = true;
        match self.should_fail {
            true => Err("".to_string()),
            false => Ok(vec![new_default_return()])
        }
    }

    fn delete_order(&self, web_connection: &dyn WebConnection, host: String, table_number: u32, order_id: u32) -> Result<(), String> {
        *self.was_delete_order_called.borrow_mut() = true;
        match self.should_fail {
            true => Err("".to_string()),
            false => Ok(())
        }
    }

    fn get_menu_items(&self, web_connection: &dyn WebConnection, host: String) -> Result<rest_responses::MenuItems, String> {
        match self.should_fail {
            true => Err("".to_string()),
            false => Ok(rest_responses::MenuItems {
                menu_items: vec![rest_responses::MenuItem {
                    id: DEFAULT_RETURN_ORDER_MENU_ID,
                    name: DEFAULT_RETURN_ORDER_MENU_NAME.to_string()
                }]
            })
        }
    }
}