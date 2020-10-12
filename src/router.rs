use confetti::{Method, Middleware, Route};

use crate::middleware::*;

pub fn routes() -> Vec<Route> {
    return vec![
        Route {
            pattern: (Method::Get, "/"),
            middlewares: vec![Middleware::new(hello)],
        },
        Route {
            pattern: (Method::Get, "/hello/json"),
            middlewares: vec![Middleware::new(data)],
        },
    ];
}
