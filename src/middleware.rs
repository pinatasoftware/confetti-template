use confetti::{html, json, Conn};

pub fn hello(conn: Conn) -> Conn {
    html(conn, "<h1>Hello World from Confetti</h1>")
}

pub fn data(conn: Conn) -> Conn {
    json(conn, "{\"message\": \"Hello World\"}")
}
