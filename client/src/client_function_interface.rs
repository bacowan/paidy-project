use crate::client_functions;
use crate::web_connection::WebConnection;
use server::rest_responses;

pub trait ClientFunctionInterface {
    fn get_all_orders(&self, web_connection: &dyn WebConnection, host: String, table_number: u32) -> Result<Vec<rest_responses::Order>, String>;
    fn get_order(&self, web_connection: &dyn WebConnection, host: String, table_number: u32, order_id: u32) -> Result<rest_responses::Order, String>;
    fn add_orders<F>(&self, web_connection: &dyn WebConnection, host: String, table_number: u32, menu_item_ids: Vec<u32>, should_retry: F) -> Result<Vec<rest_responses::Order>, String>
        where F: Fn() -> bool;
    fn delete_order(&self, web_connection: &dyn WebConnection, host: String, table_number: u32, order_id: u32) -> Result<(), String>;
    fn get_menu_items(&self, web_connection: &dyn WebConnection, host: String) -> Result<rest_responses::MenuItems, String>;
}

pub struct DefaultClientFunctionInterface {}

impl ClientFunctionInterface for DefaultClientFunctionInterface {
    fn get_all_orders(&self, web_connection: &dyn WebConnection, host: String, table_number: u32) -> Result<Vec<rest_responses::Order>, String> {
        client_functions::get_all_orders(web_connection, host, table_number)
    }

    fn get_order(&self, web_connection: &dyn WebConnection, host: String, table_number: u32, order_id: u32) -> Result<rest_responses::Order, String> {
        client_functions::get_order(web_connection, host, table_number, order_id)
    }

    fn add_orders<F>(&self, web_connection: &dyn WebConnection, host: String, table_number: u32, menu_item_ids: Vec<u32>, should_retry: F) -> Result<Vec<rest_responses::Order>, String>
        where F: Fn() -> bool {
        client_functions::add_orders(web_connection, host, table_number, menu_item_ids, should_retry)
    }

    fn delete_order(&self, web_connection: &dyn WebConnection, host: String, table_number: u32, order_id: u32) -> Result<(), String> {
        client_functions::delete_order(web_connection, host, table_number, order_id)
    }

    fn get_menu_items(&self, web_connection: &dyn WebConnection, host: String) -> Result<rest_responses::MenuItems, String> {
        client_functions::get_menu_items(web_connection, host)
    }
}