use rouille::Response;
use rust_embed::Embed;
use std::net::SocketAddr;

const PORT: u16 = 8080;

#[derive(Embed)]
#[folder = "static/"]
struct Asset;

fn main() {
    rouille::start_server(SocketAddr::from(([0, 0, 0, 0], PORT)), move |_request| {
        let index_html = Asset::get("index.html").unwrap();
        let response = Response::from_data("text/html", index_html.data);
        return response;
    });
}
