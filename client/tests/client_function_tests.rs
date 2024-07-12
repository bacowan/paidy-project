mod mock_web_connection;

#[cfg(test)]
mod tests {
    use client::{client_functions, web_connection::WebError};
    use reqwest::StatusCode;

    use crate::mock_web_connection::{self, Method, MockWebConnection};

    #[test]
    fn get_all_orders_success() -> Result<(), String> {
        let connection = MockWebConnection::new(
            Method::GET,
            StatusCode::OK,
            false,
            "{
                            \"orders\": [
                                {
                                    \"id\": 1,
                                    \"table_number\": 2,
                                    \"menu_item_id\": 3,
                                    \"menu_item_name\": \"test\",
                                    \"minutes_to_cook\": 4
                                }
                            ]
                        }".to_string()
        );
        let result = client_functions::get_all_orders(
            &connection,
            "".to_string(),
            1);
        assert!(*connection.was_get_called.borrow());
        assert!(result.is_ok());
        let orders = result.unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].id, 1);
        assert_eq!(orders[0].menu_item_id, 3);
        assert_eq!(orders[0].menu_item_name, "test");
        assert_eq!(orders[0].minutes_to_cook, 4);
        Ok(())
    }
    
    #[test]
    fn get_all_orders_failure() -> Result<(), String> {
        let connection = MockWebConnection::new(
            Method::GET,
            StatusCode::INTERNAL_SERVER_ERROR,
            false,
            "{ \"error\": \"error\" }".to_string()
        );
        let result = client_functions::get_all_orders(
            &connection,
            "".to_string(),
            1);
        assert!(*connection.was_get_called.borrow());
        assert!(result.is_err());
        Ok(())
    }
    
    #[test]
    fn get_order_success() -> Result<(), String> {
        let connection = MockWebConnection::new(
            Method::GET,
            StatusCode::OK,
            false,
            "{
                            \"id\": 1,
                            \"table_number\": 2,
                            \"menu_item_id\": 3,
                            \"menu_item_name\": \"test\",
                            \"minutes_to_cook\": 4
                        }".to_string()
        );
        let result = client_functions::get_order(
            &connection,
            "".to_string(),
            1,
        1);
        assert!(*connection.was_get_called.borrow());
        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(order.id, 1);
        assert_eq!(order.menu_item_id, 3);
        assert_eq!(order.menu_item_name, "test");
        assert_eq!(order.minutes_to_cook, 4);
        Ok(())
    }

    #[test]
    fn get_order_failure() -> Result<(), String> {
        let connection = MockWebConnection::new(
            Method::GET,
            StatusCode::INTERNAL_SERVER_ERROR,
            false,
            "{ \"error\": \"error\" }".to_string()
        );
        let result = client_functions::get_order(
            &connection,
            "".to_string(),
            1,
        1);
        assert!(*connection.was_get_called.borrow());
        assert!(result.is_err());
        Ok(())
    }
    
    #[test]
    fn add_orders_success() -> Result<(), String> {
        let connection = MockWebConnection::new(
            Method::POST,
            StatusCode::OK,
            false,
            "{
                            \"orders\": [
                                {
                                    \"id\": 1,
                                    \"table_number\": 2,
                                    \"menu_item_id\": 3,
                                    \"menu_item_name\": \"test\",
                                    \"minutes_to_cook\": 4
                                }
                            ]
                        }".to_string()
        );
        let result = client_functions::add_orders(
            &connection,
            "".to_string(),
            1,
            vec![1],
        || false);
        assert!(*connection.was_post_called.borrow());
        assert!(result.is_ok());
        let orders = result.unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].id, 1);
        assert_eq!(orders[0].menu_item_id, 3);
        assert_eq!(orders[0].menu_item_name, "test");
        assert_eq!(orders[0].minutes_to_cook, 4);
        Ok(())
    }

    
    #[test]
    fn add_order_failure() -> Result<(), String> {
        let connection = MockWebConnection::new(
            Method::GET,
            StatusCode::INTERNAL_SERVER_ERROR,
            false,
            "{ \"error\": \"error\" }".to_string()
        );
        let result = client_functions::add_orders(
            &connection,
            "".to_string(),
            1,
            vec![1],
        || false);
        assert!(*connection.was_post_called.borrow());
        assert!(result.is_err());
        Ok(())
    }

    
    #[test]
    fn add_order_retry() -> Result<(), String> {
        let connection = MockWebConnection::new(
            Method::POST,
            StatusCode::CONFLICT,
            true,
            "{ \"error\": \"error\" }".to_string()
        );

        let retry_count = std::cell::Cell::new(0);
        let result: Result<Vec<server::rest_responses::Order>, String> = client_functions::add_orders(
            &connection,
            "".to_string(),
            1,
            vec![1],
        || {
            let count = retry_count.get();
            retry_count.set(count + 1);
            count == 0
        });
        assert!(*connection.was_post_called.borrow());
        assert!(result.is_err());
        assert_eq!(retry_count.get(), 2);
        Ok(())
    }

    #[test]
    fn delete_orders_success() -> Result<(), String> {
        let connection = MockWebConnection::new(
            Method::DELETE,
            StatusCode::NO_CONTENT,
            false,
            "{}".to_string()
        );
        let result = client_functions::delete_order(
            &connection,
            "".to_string(),
            1,
        1);
        assert!(*connection.was_delete_called.borrow());
        assert!(result.is_ok());
        Ok(())
    }
    
    #[test]
    fn delete_failure() -> Result<(), String> {
        let connection = MockWebConnection::new(
            Method::DELETE,
            StatusCode::INTERNAL_SERVER_ERROR,
            false,
            "{ \"error\": \"error\" }".to_string()
        );
        let result = client_functions::delete_order(
            &connection,
            "".to_string(),
            1,
        1);
        assert!(result.is_err());
        Ok(())
    }
}