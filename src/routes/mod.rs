use std::io::{Read, Write};
use std::net::TcpStream;
use crate::handlers::user_handler;

const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    if let Ok(size) = stream.read(&mut buffer) {
        request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

        let (status_line, content) = match request.as_str() {
            r if r.starts_with("POST /users") => user_handler::handle_post_request(r),
            r if r.starts_with("GET /users/") => user_handler::handle_get_request(r),
            r if r.starts_with("GET /users") => user_handler::handle_get_all_request(r),
            r if r.starts_with("PUT /users/") => user_handler::handle_put_request(r),
            r if r.starts_with("DELETE /users/") => user_handler::handle_delete_request(r),
            _ => (NOT_FOUND.to_string(), "404 Not Found".to_string()),
        };

        let response = format!("{}{}", status_line, content);
        stream.write_all(response.as_bytes()).unwrap();
    }
}
