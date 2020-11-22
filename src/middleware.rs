use confetti::{html, json, Conn};
use std::{future::Future, pin::Pin};

// For now we need t pin each middleware before using it in the router
pub fn hello(c: Conn) -> Pin<Box<dyn Future<Output = Conn>>> {
    Box::pin(hello_html(c))
}

pub fn data(c: Conn) -> Pin<Box<dyn Future<Output = Conn>>> {
    Box::pin(data_json(c))
}

pub async fn hello_html(conn: Conn) -> Conn {
    html(conn, "<h1>Hello World from Confetti</h1>")
}

pub async fn data_json(conn: Conn) -> Conn {
    json(conn, "{\"message\": \"Hello World\"}")
}
