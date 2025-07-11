use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::Arc;

use crate::web::response::{ResponseCode};
use crate::web::request::Request;
use crate::worker_pool::Pool;

pub mod request;
pub mod response;

/// Disambiguation for [RouteMap]
type Uri = String;
/// Disambiguation for [RouteMap]
type Method = String;
/// A shared (across threads) pointer to the fn or closure 
/// that gets executed during HTTP response. Used in [RouteMap].
type Action = Arc<dyn Fn(Request)>;

/// Public api for registering actions to method + URI combinations (called routes).
/// After all routes are registered, the RouteMap is to be consumed by a [Server].
/// insert_* methods follow the [HashMap] rules for overwriting keys.
/// [insert_route_methods] is a shorthand for registering the same 
/// action to multiple methods under the same URI.
/// # Example
/// ```
/// use std::sync::Arc;
/// 
/// use rns::web::request::Request;
/// use rns::web::RouteMap;
/// 
/// let mut route_map = RouteMap::new();
/// 
/// fn dummy_function(request: Request) {}
/// 
/// let fn_ptr = Arc::new(dummy_function);
/// 
/// route_map.insert_route(
///     "/test".to_string(),
///     "GET".to_string(),
///     fn_ptr
/// );
/// 
/// let fn_ptr_return_result = route_map.get_action(
///     &"/test".to_string(),
///     &"GET".to_string()
/// );
/// ```
pub struct RouteMap {
    map: HashMap<
        Uri,
        HashMap<
            Method,
            Action
        >
    >
}

impl RouteMap {
    pub fn new() -> RouteMap {
        RouteMap { map: HashMap::new() }
    }

    /// Inserts one route. 
    /// # Parameters
    /// uri - a uri string to be consumed (be owned by underlying HashMap).
    /// method - a method string to be consumed (be owned by underlying HashMap).
    /// closure - [Rc] to a fn or closure that will be executed. Gets cloned.
    pub fn insert_route(&mut self, uri: String, method: String, closure: Action) {
        let method_map = self.get_method_map(uri);
        method_map.insert(method, closure.clone());
    }

    /// Inserts multiple routes with the same Uri but different methods. 
    /// # Parameters
    /// uri - a uri string to be consumed (be owned by underlying HashMap).
    /// method - a vector of method strings that gets drained (ownership
    /// of Strings goes to underlying HashMap).
    /// closure - [Rc] to a fn or closure that will be executed. Gets cloned.
    pub fn insert_route_methods(&mut self, uri: String, methods: &mut Vec<String>, closure: Action) {
        let method_map = self.get_method_map(uri);
      
        for method in methods.drain(..) {
            method_map.insert(method, closure.clone());
        }
    }

    /// Returns action pointer or [ResponseCode] of [404, 405]
    /// that can be used for a meaningful http response.
    pub fn get_action(&self, uri: &String, method: &String) -> Result<Action, ResponseCode> {
        // Check for uri
        match self.map.get(uri) {
            Some(method_map) => {
                // Chech for method
                match method_map.get(method) {
                    Some(action) => {
                        // Ok
                        Result::Ok(action.clone())
                    }
                    None => {
                        // uri exists but method is not registered => Method Not Allowed
                        Result::Err(
                            ResponseCode::get_405()
                        )
                    }
                }
            }
            None => {
                // No uri => Not Found
                Result::Err(
                    ResponseCode::get_404()
                )
            }
        }
    }

    /// Private helper method to get or create method map
    fn get_method_map(&mut self, uri: String) -> &mut HashMap<Method, Action> {
        if let None = self.map.get(&uri) {
            let method_map = HashMap::new();
            self.map.insert(uri.clone(), method_map);
        }

        // Now uri key exists, so unwrap() is safe.
        self.map.get_mut(&uri).unwrap()
    }
}

/// Implements the non-public interface of a webserver.
/// [serve_request] invokes the request processing chain
/// that goes in order of:
/// [authenticate] -> [throttle] -> [dispatch] -> closure().
/// The [serve_request] method also uses a shared read only
/// [RouteMap] to look up the closure at the end of the chain.
trait ServerBackend {
    fn authenticate(request: &Request) -> Result<(), ResponseCode>;
    fn throttle(request: &Request) -> Result<(), ResponseCode>;
    fn dispatch(request: &Request) -> Result<impl Fn(Request), ResponseCode>;

    fn get_route_map(&mut self) -> Arc<RouteMap>;
    fn serve_request(socket: TcpStream, route_map: Arc<RouteMap>);
}

/// Implements the public interface of a webserver.
/// [run] method will start a blocking infinite loop
/// that awaits connections and handles them.
pub trait Server {
    fn run(&self);
}

impl<T: ServerBackend> Server for T {
    fn run(&self) {
        
    }
}

/// Server + ServerBackend with a pool of workers to
/// process connections concurrently. 
pub struct PooledServer {
    routes: RouteMap,
    worker_pool: Pool
}

impl ServerBackend for PooledServer {
    fn authenticate(request: &Request) -> Result<(), ResponseCode> {
        Ok(())
    }

    fn throttle(request: &Request) -> Result<(), ResponseCode> {
        Ok(())
    }

    fn dispatch(request: &Request) -> Result<impl Fn(Request), ResponseCode> {
        Ok(|request| {})
    }

    fn get_route_map(&mut self) -> Arc<RouteMap> {
        Arc::new(RouteMap::new())
    }

    fn serve_request(socket: TcpStream, route_map: Arc<RouteMap>) {
        let request = Request::build(&socket).unwrap();

        if let Err(status) = <PooledServer as ServerBackend>::authenticate(&request) {

        }

        if let Err(status) = <PooledServer as ServerBackend>::throttle(&request) {
            
        }

        match <PooledServer as ServerBackend>::dispatch(&request) {
            Ok(closure) => {

            }
            Err(status) => {

            }
        }

    }
}

#[cfg(test)]
mod tests;
