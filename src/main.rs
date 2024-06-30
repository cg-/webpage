use rouille::Request;
use rouille::Response;
use std::net::SocketAddr;

const PORT: u16 = 8080;

fn main() {
    rouille::start_server(SocketAddr::from(([0, 0, 0, 0], PORT)), move |request| {
        Response::text("hello world")
    });
}
