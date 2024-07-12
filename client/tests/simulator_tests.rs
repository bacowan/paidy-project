mod mock_client_function_interface;

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use client::sim::{self, TableOrderPair};
    use rand::{rngs::StdRng, SeedableRng};
    use crate::mock_client_function_interface::{self, MockClientFunctionInterface};

    fn get_mock_injection_params() -> sim::SimInjectionParams<MockClientFunctionInterface> {
        sim::SimInjectionParams {
            client_functions: mock_client_function_interface::new(),
            rng: StdRng::seed_from_u64(0)
        }
    }
    
    #[test]
    fn add_random_order_success() {
        // setup
        let mut injection_params = get_mock_injection_params();
        let mut added_item_cache = Vec::<TableOrderPair>::new();

        // execute
        let ret = sim::add_random_order(&mut injection_params, 1, &mut added_item_cache);
        
        // assert
        assert!(ret.contains(mock_client_function_interface::DEFAULT_RETURN_ORDER_MENU_NAME));
        assert_eq!(added_item_cache.len(), 1);
        assert_eq!(added_item_cache.get(0).unwrap().order_id, mock_client_function_interface::DEFAULT_RETURN_ORDER_ID);
        assert!(*injection_params.client_functions.was_add_orders_called.borrow());
    }
    
    #[test]
    fn add_random_order_failure() {
        // setup
        let mut injection_params = get_mock_injection_params();
        injection_params.client_functions.should_fail = true;
        let mut added_item_cache = Vec::<TableOrderPair>::new();

        // execute
        let ret = sim::add_random_order(&mut injection_params, 1, &mut added_item_cache);
        
        // assert
        assert!(ret.contains("error"));
    }
    
    #[test]
    fn delete_random_order_success() {
        // setup
        let mut injection_params = get_mock_injection_params();
        let mut added_item_cache = vec![TableOrderPair {
            table_id: 3,
            order_id: 2
        }];

        // execute
        let ret = sim::delete_random_order(&mut injection_params, 1, &mut added_item_cache);
        
        // assert
        assert!(ret.contains("deleted"));
        assert_eq!(added_item_cache.len(), 0);
        assert!(*injection_params.client_functions.was_delete_order_called.borrow());
    }
    
    #[test]
    fn delete_random_order_failure() {
        // setup
        let mut injection_params = get_mock_injection_params();
        *injection_params.client_functions.should_fail.borrow_mut() = true;
        let mut added_item_cache = vec![TableOrderPair {
            table_id: 3,
            order_id: 2
        }];

        // execute
        let ret = sim::delete_random_order(&mut injection_params, 1, &mut added_item_cache);
        
        // assert
        assert!(ret.contains("error"));
        assert_eq!(added_item_cache.len(), 1);
    }
    
    #[test]
    fn delete_random_order_nothing_to_delete() {
        // setup
        let mut injection_params = get_mock_injection_params();
        let mut added_item_cache = Vec::<TableOrderPair>::new();

        // execute
        let ret = sim::delete_random_order(&mut injection_params, 1, &mut added_item_cache);
        
        // assert
        assert!(ret.contains("none to delete"));
    }
    
    #[test]
    fn query_random_table_success() {
        // setup
        let mut injection_params = get_mock_injection_params();

        // execute
        let ret = sim::query_random_table(&mut injection_params, 1);
        
        // assert
        assert!(ret.contains("including"));
        assert!(ret.contains(mock_client_function_interface::DEFAULT_RETURN_ORDER_MENU_NAME));
        assert!(ret.contains(mock_client_function_interface::DEFAULT_RETURN_ORDER_MINUTES_TO_COOK.to_string().as_str()));
    }
    
    #[test]
    fn query_random_table_failure() {
        // setup
        let mut injection_params = get_mock_injection_params();
        *injection_params.client_functions.should_fail.borrow_mut() = true;

        // execute
        let ret = sim::query_random_table(&mut injection_params, 1);
        
        // assert
        assert!(ret.contains("error"));
    }

    #[test]
    fn query_random_table_item_success() {
        // setup
        let mut injection_params = get_mock_injection_params();
        let mut added_item_cache = vec![TableOrderPair {
            table_id: 3,
            order_id: 2
        }];

        // execute
        let ret = sim::query_random_table_item(&mut injection_params, 1, &mut added_item_cache);
        
        // assert
        assert!(!ret.contains("including"));
        assert!(ret.contains(mock_client_function_interface::DEFAULT_RETURN_ORDER_MENU_NAME));
        assert!(ret.contains(mock_client_function_interface::DEFAULT_RETURN_ORDER_MINUTES_TO_COOK.to_string().as_str()));
    }
    
    #[test]
    fn query_random_table_item_failure() {
        // setup
        let mut injection_params = get_mock_injection_params();
        *injection_params.client_functions.should_fail.borrow_mut() = true;
        let mut added_item_cache = vec![TableOrderPair {
            table_id: 3,
            order_id: 2
        }];

        // execute
        let ret = sim::query_random_table_item(&mut injection_params, 1, &mut added_item_cache);
        
        // assert
        assert!(ret.contains("error"));
    }

    #[test]
    fn query_random_table_item_none_to_query() {
        // setup
        let mut injection_params = get_mock_injection_params();
        let mut added_item_cache = Vec::<TableOrderPair>::new();

        // execute
        let ret = sim::query_random_table_item(&mut injection_params, 1, &mut added_item_cache);
        
        // assert
        assert!(ret.contains("none to query"));
    }
}