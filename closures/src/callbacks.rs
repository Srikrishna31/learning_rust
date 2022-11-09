use std::collections::HashMap;

pub(crate) struct Request {
    pub(crate) method: String,
    pub(crate) url: String,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: Vec<u8>,
}

pub(crate) struct Response {
    pub(crate) code: u32,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: Vec<u8>,
}

type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

pub(crate) struct BasicRouter {
    routes: HashMap<String, BoxedCallback>
}

/// Closures have unique types because each one captures different variables, so among other things,
/// they're each a different size. If they don't capture anything, though, there's nothing to store.
/// By using fn pointers in functions that take callbacks, you can restrict a caller to use only
/// these noncapturing closures, gaining some performance and flexibility within the code using
/// callbacks at the cost of flexibility for the users of your API.
impl BasicRouter where {
    /// Create an empty router
    pub(crate) fn new() -> BasicRouter {
        BasicRouter { routes: HashMap::new() }
    }

    /// Add a route to the router.
    /// Note the two bounds on C in the type signature for add_route: a particular Fn trait and the
    /// 'static lifetime. Rust makes us add this 'static bound. Without it, the call to Box::new(callback)
    /// would be an error, because it's not safe to store a closure if it contains borrowed references
    /// to variables that are about to go out of scope.
    pub(crate) fn add_route<C>(&mut self, url: &str, callback: C)
        where C: Fn(&Request) -> Response + 'static {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

    pub(crate) fn handle_request(&self, request:&Request) -> Response {
        match self.routes.get(&request.url) {
            None => not_found_response(),
            Some(callback) => callback(request),
        }
    }
}

pub(crate) fn get_form_response() -> Response {
    Response {
        code: 200,
        headers: HashMap::from([("text/type".to_string(), "json".to_string()),
            ("body".to_string(), "<HTML></HTML>".to_string())]),
        body: Vec::from("empty"),
    }
}

pub(crate) fn get_gcd_response(request: &Request) -> Response {
    Response {
        code: 200,
        headers: HashMap::from([("text/type".to_string(), "json".to_string()),
            ("body".to_string(), "<HTML></HTML>".to_string())]),
        body: Vec::from("empty"),
    }
}

fn not_found_response() -> Response {
    Response {
        code: 404,
        headers: HashMap::from([("text/type".to_string(), "json".to_string()),
            ("body".to_string(), "<HTML></HTML>".to_string())]),
        body: Vec::from("Not found"),
    }
}
