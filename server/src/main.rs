#[macro_use] extern crate rocket;

use rocket::{ routes, launch };

use server::server_functions;
use server::database_connector::{ DatabaseConnector, DefaultDatabaseConnector };
use server::endpoints::*;

const DATABASE_PATH: &str = "database.db";

#[launch]
fn rocket() -> _ {
    let database_connector = DefaultDatabaseConnector {
        path: DATABASE_PATH.to_string()
    };
    match server_functions::setup_database(&database_connector) {
        Ok(_) => {},
        Err(err) => panic!("Failed to setup database: {}", err),
    };
    rocket::build()
        .manage(Box::new(database_connector) as Box<dyn DatabaseConnector>)
        .mount("/", routes![get_table_orders])
        .mount("/", routes![post_table_order])
        .mount("/", routes![get_table_order])
        .mount("/", routes![delete_table_order])
        .mount("/", routes![get_menu_items])
        .register("/", catchers![internal_error, not_found, default, unprocessable_entity])
}