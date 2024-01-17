use server::Server;
use website_handler::WebsiteHandler;
use std::env;

mod server;
mod http;
mod website_handler;


fn main() {
    let default_path: String = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    println!("public path {}", default_path);
    
    let server = Server::new("127.0.0.1.:8080".to_string());
    server.run(WebsiteHandler::new(default_path)); 
}