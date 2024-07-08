#[cfg(test)]

mod mock_database_connector;

mod tests {
    use rocket::http::{ContentType, Header, Status};
    use rocket::serde::Deserialize;
    use server::database_connector::{ self, DatabaseConnector };
    use server::server_errors::ServerError;
    use rocket::local::blocking::{Client, LocalResponse};
    use server::server_functions::setup_database;
    use rocket::serde::json::{ Json, to_string, from_str };
    use server::{rest_bodies, rest_responses};
    use rocket::{catchers, response, routes, serde};
    use tempfile::NamedTempFile;

    use crate::mock_database_connector::{self, MockDatabaseConnector};
    use server::endpoints::*;

    #[derive(Deserialize)]
    #[serde(crate = "rocket::serde")]
    struct ErrorResponse {
        pub error: String
    }

    fn assert_response_contains_error(response: LocalResponse) -> Result<ErrorResponse, String> {
        from_str::<ErrorResponse>(&response.into_string().unwrap())
            .map_err(|e| e.to_string())
    }

    fn create_client_without_setup() -> Result<Client, String> {
        let database_connector = mock_database_connector::new()?;
        let rocket = rocket::build()
            .mount("/", routes![get_table_orders])
            .mount("/", routes![post_table_order])
            .mount("/", routes![get_table_order])
            .mount("/", routes![delete_table_order])
            .mount("/", routes![get_menu_items])
            .register("/", catchers![internal_error, not_found, default, unprocessable_entity])
            .manage(Box::new(database_connector) as Box<dyn DatabaseConnector>);
        Ok(Client::tracked(rocket).unwrap())
    }

    fn create_client() -> Result<Client, String> {
        let database_connector = mock_database_connector::new()?;
        setup_database(&database_connector).map_err(|e| e.to_string())?;
        let rocket = rocket::build()
            .mount("/", routes![get_table_orders])
            .mount("/", routes![post_table_order])
            .mount("/", routes![get_table_order])
            .mount("/", routes![delete_table_order])
            .mount("/", routes![get_menu_items])
            .register("/", catchers![internal_error, not_found, default, unprocessable_entity])
            .manage(Box::new(database_connector) as Box<dyn DatabaseConnector>);
        Ok(Client::tracked(rocket).unwrap())
    }

    #[test]
    fn setup_database_no_error() -> Result<(), String> {
        // setup
        let database_connector = mock_database_connector::new()?;

        // execution
        let result = setup_database(&database_connector);

        // assertion
        assert!(result.is_ok(), "Error setting up database: {}", result.unwrap_err().to_string());
        Ok(())
    }

    #[test]
    fn menu_items_get_has_default_data() -> Result<(), String> {
        // setup
        let client = create_client()?;

        // execution
        let req = client.get("/menu-items");
        let response = req.dispatch();
        
        // assertion
        assert_eq!(response.status(), Status::Ok, "Call to /menu-items failed");
        let menu_items = from_str::<rest_responses::MenuItems>(&response.into_string().unwrap())
            .map_err(|e| e.to_string())
            ?.menu_items;
        assert_eq!(menu_items.len(), 5, "Unexpected number of menu items were initialized");
        assert!(menu_items.iter().find(|m| m.name == "Hamburger").is_some());
        assert!(menu_items.iter().find(|m| m.name == "Salad").is_some());
        assert!(menu_items.iter().find(|m| m.name == "Sushi").is_some());
        assert!(menu_items.iter().find(|m| m.name == "Ice Cream").is_some());
        assert!(menu_items.iter().find(|m| m.name == "Soda").is_some());
        Ok(())
    }

    #[test]
    fn orders_post() -> Result<(), String> {
        // setup
        let client = create_client()?;
        let orders = rest_bodies::Orders {
            orders: vec![
                rest_bodies::Order {
                    menu_item_id: 1,
                    idempotency_key: "key1".to_string()
                },
                rest_bodies::Order {
                    menu_item_id: 2,
                    idempotency_key: "key2".to_string()
                }
            ]
        };

        // execution
        let post_req = client.post("/tables/1/orders")
            .header(ContentType::JSON)
            .body(to_string(&orders).map_err(|e| e.to_string())?);
        let post_response = post_req.dispatch();

        // assertion
        assert_eq!(post_response.status(), Status::Ok);
        let post_orders = from_str::<rest_responses::Orders>(&post_response.into_string().unwrap())
            .map_err(|e| e.to_string())
            ?.orders;
        assert_eq!(post_orders.len(), 2);
        assert!(post_orders.iter().find(|o| o.menu_item_id == 1).is_some());
        assert!(post_orders.iter().find(|o| o.menu_item_id == 2).is_some());
        Ok(())
    }

    #[test]
    fn orders_get_single() -> Result<(), String> {
        // setup
        let client = create_client()?;
        let orders = rest_bodies::Orders {
            orders: vec![
                rest_bodies::Order {
                    menu_item_id: 1,
                    idempotency_key: "key1".to_string()
                },
                rest_bodies::Order {
                    menu_item_id: 2,
                    idempotency_key: "key2".to_string()
                }
            ]
        };

        // setup with POST call
        let post_req = client.post("/tables/1/orders")
            .header(ContentType::JSON)
            .body(to_string(&orders).map_err(|e| e.to_string())?);
        let post_response = post_req.dispatch();
        let post_orders = from_str::<rest_responses::Orders>(&post_response.into_string().unwrap())
            .map_err(|e| e.to_string())
            ?.orders;
        let order = post_orders.iter().find(|o| o.menu_item_id == 1).unwrap();

        // execution
        let get_req = client.get(format!("/tables/1/orders/{}", order.id));
        let get_response = get_req.dispatch();

        // assertion
        assert_eq!(get_response.status(), Status::Ok);
        let order = from_str::<rest_responses::Order>(&get_response.into_string().unwrap())
            .map_err(|e| e.to_string())?;
        assert_eq!(order.menu_item_id, 1);
        assert!(!order.menu_item_name.is_empty());
        assert!(order.minutes_to_cook >= 5);
        assert!(order.minutes_to_cook <= 15);
        Ok(())
    }

    #[test]
    fn orders_get_multiple() -> Result<(), String> {
        // setup
        let client = create_client()?;
        let orders = rest_bodies::Orders {
            orders: vec![
                rest_bodies::Order {
                    menu_item_id: 1,
                    idempotency_key: "key1".to_string()
                },
                rest_bodies::Order {
                    menu_item_id: 2,
                    idempotency_key: "key2".to_string()
                }
            ]
        };

        // setup with POST call
        let post_req = client.post("/tables/1/orders")
            .header(ContentType::JSON)
            .body(to_string(&orders).map_err(|e| e.to_string())?);
        post_req.dispatch();

        // execution
        let get_req = client.get("/tables/1/orders");
        let get_response = get_req.dispatch();

        // assertion
        assert_eq!(get_response.status(), Status::Ok);
        let orders = from_str::<rest_responses::Orders>(&get_response.into_string().unwrap())
            .map_err(|e| e.to_string())?
            .orders;

        let first_order = orders.iter().find(|o| o.menu_item_id == 1).unwrap();
        assert_eq!(first_order.menu_item_id, 1);
        assert!(!first_order.menu_item_name.is_empty());
        assert!(first_order.minutes_to_cook >= 5);
        assert!(first_order.minutes_to_cook <= 15);

        let second_order = orders.iter().find(|o| o.menu_item_id == 2).unwrap();
        assert_eq!(second_order.menu_item_id, 2);
        assert!(!second_order.menu_item_name.is_empty());
        assert!(second_order.minutes_to_cook >= 5);
        assert!(second_order.minutes_to_cook <= 15);
        Ok(())
    }

    #[test]
    fn orders_delete() -> Result<(), String> {
        // setup
        let client = create_client()?;
        let orders = rest_bodies::Orders {
            orders: vec![
                rest_bodies::Order {
                    menu_item_id: 1,
                    idempotency_key: "key1".to_string()
                }
            ]
        };

        // setup with POST call
        let post_req = client.post("/tables/1/orders")
            .header(ContentType::JSON)
            .body(to_string(&orders).map_err(|e| e.to_string())?);
        let post_response = post_req.dispatch();
        let post_orders = from_str::<rest_responses::Orders>(&post_response.into_string().unwrap())
            .map_err(|e| e.to_string())
            ?.orders;
        let order = post_orders.iter().find(|o| o.menu_item_id == 1).unwrap();

        // delete execution
        let delete_req = client.delete(format!("/tables/1/orders/{}", order.id));
        let delete_response = delete_req.dispatch();

        // get execution
        let get_req = client.get(format!("/tables/1/orders/{}", order.id));
        let get_response = get_req.dispatch();

        // assertion
        assert_eq!(delete_response.status(), Status::NoContent);
        assert_eq!(get_response.status(), Status::NotFound);
        Ok(())
    }

    #[test]
    fn menu_items_error_500() -> Result<(), String> {
        // setup
        let client = create_client_without_setup()?;

        // execution
        let req = client.get("/menu-items");
        let response = req.dispatch();
        
        // assertion
        assert_eq!(response.status(), Status::InternalServerError);
        assert_response_contains_error(response)?;
        Ok(())
    }

    #[test]
    fn orders_get_multiple_error_500() -> Result<(), String> {
        // setup
        let client = create_client_without_setup()?;

        // execution
        let req = client.get("/tables/1/orders");
        let response = req.dispatch();
        
        // assertion
        assert_eq!(response.status(), Status::InternalServerError);
        assert_response_contains_error(response)?;
        Ok(())
    }

    #[test]
    fn orders_post_error_400() -> Result<(), String> {
        // setup
        let client = create_client_without_setup()?;
        let orders = rest_bodies::Orders {
            orders: vec![
                rest_bodies::Order {
                    menu_item_id: 1,
                    idempotency_key: "key1".to_string()
                },
                rest_bodies::Order {
                    menu_item_id: 2,
                    idempotency_key: "key2".to_string()
                }
            ]
        };

        // execution
        let post_req = client.post("/tables/1/orders")
            .header(ContentType::JSON)
            .body(to_string(&orders).map_err(|e| e.to_string())?);
        let post_response = post_req.dispatch();
        
        // assertion
        assert_eq!(post_response.status(), Status::InternalServerError);
        assert_response_contains_error(post_response)?;
        Ok(())
    }

    #[test]
    fn orders_post_error_422() -> Result<(), String> {
        // setup
        let client = create_client()?;

        // execution
        let post_req = client.post("/tables/1/orders")
            .header(ContentType::JSON)
            .body("{ \"orders\": [{ \"idempotency_key\": 1 " );
        let post_response = post_req.dispatch();
        
        // assertion
        assert_eq!(post_response.status(), Status::UnprocessableEntity);
        assert_response_contains_error(post_response)?;
        Ok(())
    }

    #[test]
    fn orders_post_error_409() -> Result<(), String> {
        // setup
        let client = create_client()?;
        let orders = rest_bodies::Orders {
            orders: vec![
                rest_bodies::Order {
                    menu_item_id: 1,
                    idempotency_key: "key".to_string()
                },
                rest_bodies::Order {
                    menu_item_id: 2,
                    idempotency_key: "key".to_string()
                }
            ]
        };

        // execution
        let post_req = client.post("/tables/1/orders")
            .header(ContentType::JSON)
            .body(to_string(&orders).map_err(|e| e.to_string())?);
        let post_response = post_req.dispatch();
        
        // assertion
        assert_eq!(post_response.status(), Status::Conflict);
        assert_response_contains_error(post_response)?;
        Ok(())
    }
    

    #[test]
    fn orders_post_error_500() -> Result<(), String> {
        // setup
        let client = create_client_without_setup()?;
        let orders = rest_bodies::Orders {
            orders: vec![
                rest_bodies::Order {
                    menu_item_id: 1,
                    idempotency_key: "key".to_string()
                },
                rest_bodies::Order {
                    menu_item_id: 2,
                    idempotency_key: "key".to_string()
                }
            ]
        };

        // execution
        let post_req = client.post("/tables/1/orders")
            .header(ContentType::JSON)
            .body(to_string(&orders).map_err(|e| e.to_string())?);
        let post_response = post_req.dispatch();
        
        // assertion
        assert_eq!(post_response.status(), Status::InternalServerError);
        assert_response_contains_error(post_response)?;
        Ok(())
    }
    

    #[test]
    fn orders_get_single_error_404() -> Result<(), String> {
        // setup
        let client = create_client()?;

        // execution
        let req = client.get("/tables/1/orders/1");
        let response = req.dispatch();
        
        // assertion
        assert_eq!(response.status(), Status::NotFound);
        assert_response_contains_error(response)?;
        Ok(())
    }
    

    #[test]
    fn orders_get_single_different_table_error_404() -> Result<(), String> {
        // setup
        let client = create_client()?;
        let orders = rest_bodies::Orders {
            orders: vec![
                rest_bodies::Order {
                    menu_item_id: 1,
                    idempotency_key: "key1".to_string()
                },
                rest_bodies::Order {
                    menu_item_id: 2,
                    idempotency_key: "key2".to_string()
                }
            ]
        };

        // post execution
        let post_req = client.post("/tables/1/orders")
            .header(ContentType::JSON)
            .body(to_string(&orders).map_err(|e| e.to_string())?);
        post_req.dispatch();

        // get execution
        let req = client.get("/tables/2/orders/1");
        let response = req.dispatch();
        
        // assertion
        assert_eq!(response.status(), Status::NotFound);
        assert_response_contains_error(response)?;
        Ok(())
    }
    

    #[test]
    fn orders_get_single_error_500() -> Result<(), String> {
        // setup
        let client = create_client_without_setup()?;

        // execution
        let req = client.get("/tables/1/orders/1");
        let response = req.dispatch();
        
        // assertion
        assert_eq!(response.status(), Status::InternalServerError);
        assert_response_contains_error(response)?;
        Ok(())
    }
    

    #[test]
    fn orders_delete_error_500() -> Result<(), String> {
        // setup
        let client = create_client_without_setup()?;

        // execution
        let req = client.delete("/tables/1/orders/1");
        let response = req.dispatch();
        
        // assertion
        assert_eq!(response.status(), Status::InternalServerError);
        assert_response_contains_error(response)?;
        Ok(())
    }
}