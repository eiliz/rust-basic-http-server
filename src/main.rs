#![allow(dead_code)]
// the ! bang sign means apply this to the entire module and its submodules as opposed to having no bang sign when it
// means it will be applied only to the line following it

// pulling modules === copy/pasting the modules text into this file
mod http;
mod server;
mod website_handler;

// pulling the module's content into this file is not enough
// you have to also explicitly say what you want to use from them
use server::Server;
use website_handler::WebsiteHandler;
use std::env;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    // env! is a macro that allows reading compile time env variables
    // cargo expand will output the source code with the macros expanded into code
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path.to_string());
    println!("The public path is: {}", public_path);
    server.run(WebsiteHandler::new(public_path));
}
