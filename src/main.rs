mod routes;
mod db;
mod handlers;
mod models;

use std::net::TcpListener;

fn main() {
    if let Err(err) = db::connection::set_database() {
        eprintln!("Error setting up the database: {}", err);
        return;
    }

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            routes::handle_client(stream);
        }
    }
}
