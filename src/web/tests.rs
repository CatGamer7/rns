use std::cell::Cell;

use super::*;

#[test]
/// Test [RouteMap] with one method and one uri per action.
/// Both closure and standard function are tested.
/// Test only ensures that *some* function was stored.
fn route_map_single() {
    let mut route_map = RouteMap::new();

    fn dummy_function(request: Request) {}

    let fn_ptr = Arc::new(dummy_function);
    let cl_ptr = Arc::new(|request: Request| {});

    let uri_1 = "/test-fn".to_string();
    let uri_2 = "/test-cl".to_string();
    let method = "GET".to_string();

    route_map.insert_route(
        uri_1.clone(),
        method.clone(),
        fn_ptr
    );
    route_map.insert_route(
        uri_2.clone(),
        method.clone(),
        cl_ptr
    );

    let fn_ptr_return = route_map.get_action(
        &uri_1, 
        &method
    ).unwrap();
    let cl_ptr_return = route_map.get_action(
        &uri_1, 
        &method
    ).unwrap();
}

#[test]
/// Test [RouteMap] with multiple methods and one uri.
fn route_map_multi() {
    let mut route_map = RouteMap::new();

    fn dummy_function(request: Request) {

    }

    let fn_ptr = Arc::new(dummy_function);
    let uri = "/test-fn".to_string();
    let mut methods = vec![
        "GET".to_string(),
        "POST".to_string()
    ];

    route_map.insert_route_methods(uri.clone(), &mut methods, fn_ptr);
    
    let fn_ptr_return_1 = route_map.get_action(
        &uri, 
        &"GET".to_string()
    ).unwrap();
    let fn_ptr_return_2 = route_map.get_action(
        &uri, 
        &"POST".to_string()
    ).unwrap();
}

#[test]
/// Test [RouteMap] with negative cases: 404 and 405 return codes.
fn route_map_negative() {
    let mut route_map = RouteMap::new();

    fn dummy_function(request: Request) {

    }

    let fn_ptr = Arc::new(dummy_function);
    let uri = "/test-fn".to_string();
    let method = "GET".to_string();

    route_map.insert_route(uri.clone(), method, fn_ptr);
    
    // Should return action
    let fn_ptr_return_1 = route_map.get_action(
        &uri, 
        &"GET".to_string()
    ).unwrap();

    // Should return 404
    let return_404 = route_map.get_action(
        &"/test-non-existant".to_string(),
        &"GET".to_string()
    );
    match return_404 {
        Ok(_) => assert!(false, "should be 404 Not Found"),
        Err(code) => assert!(
            code == ResponseCode::get_404(),
            "should be 404 Not Found"
        )
    }

    // Should return 405
    let return_405 = route_map.get_action(
        &uri, 
        &"POST".to_string()
    );
    match return_405 {
        Ok(_) => assert!(false, "should be 405 Method Not Allowed"),
        Err(code) => assert!(
            code == ResponseCode::get_405(),
            "should be 405 Method Not Allowed"
        )
    }
}
