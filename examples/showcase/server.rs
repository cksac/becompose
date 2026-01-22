//! Simple static file server for the showcase web demo
//! Usage: cargo run --bin server

use std::fs;
use std::io::Read;
use std::path::PathBuf;
use tiny_http::{Response, Server};

fn main() {
    let server = Server::http("0.0.0.0:8080").unwrap();
    println!("Serving showcase at http://localhost:8080");
    println!("Open http://localhost:8080/index.html in your browser.");

    for request in server.incoming_requests() {
        let url = request.url();
        let path = if url == "/" { "index.html" } else { &url[1..] };
        let mut file_path = PathBuf::from("examples/showcase");
        file_path.push(path);

        match fs::File::open(&file_path) {
            Ok(mut file) => {
                let mut buf = Vec::new();
                file.read_to_end(&mut buf).unwrap();
                let mime = if path.ends_with(".html") {
                    "text/html"
                } else if path.ends_with(".js") {
                    "application/javascript"
                } else if path.ends_with(".wasm") {
                    "application/wasm"
                } else if path.ends_with(".css") {
                    "text/css"
                } else {
                    "application/octet-stream"
                };
                let response = Response::from_data(buf).with_header(
                    tiny_http::Header::from_bytes(&b"Content-Type"[..], mime.as_bytes()).unwrap(),
                );
                let _ = request.respond(response);
            }
            Err(_) => {
                let response = Response::from_string("404 Not Found").with_status_code(404);
                let _ = request.respond(response);
            }
        }
    }
}
