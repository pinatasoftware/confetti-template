use confetti::{Method, Route};

use crate::middleware::*;

pub fn routes() -> Vec<Route> {
    return vec![
        Route {
            pattern: (Method::Get, "/"),
            middlewares: vec![hello],
        },
        Route {
            pattern: (Method::Get, "/hello/json"),
            middlewares: vec![data],
        },
    ];
}
