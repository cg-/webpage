#![allow(unreachable_code)]
#[macro_use]
extern crate rouille;
use rust_embed::Embed;
use std::net::SocketAddr;

const PORT: u16 = 8080;

#[derive(Embed)]
#[folder = "static/"]
struct Asset;

fn main() {
    rouille::start_server(SocketAddr::from(([0, 0, 0, 0], PORT)), move |request| {
        router!(request,
            (GET) (/) => {
                rouille::Response::redirect_302("/index.html")
            },

            (GET) (/{id: String}) => {
                match Asset::get(&id){
                    // todo: add other content types based on extension
                    Some(file) => {
                        rouille::Response::from_data("text/html", file.data)
                    },
                    None => {
                        let mut r = rouille::Response::empty_404();
                        r.data = rouille::ResponseBody::from_string(format!("Couldn't find the file: {}", &id));
                        r
                    }
                }
            },

            _ => rouille::Response::empty_404()
        )
    });
}
