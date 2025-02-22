extern crate dotenv;

mod api;
mod entities;
mod repository;
mod util;

fn main() {
    // read .env file
    dotenv::dotenv().expect("Failed to read .env file");

    // start server
    match api::start() {
        Ok(_) => println!("Server started"),
        Err(e) => eprintln!("Error starting server: {:?}", e),
    }
}
