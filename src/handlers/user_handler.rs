use crate::db::connection::DB_URL;
use crate::models::user::User;
use postgres::{Client, NoTls};

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";

pub fn handle_post_request(request: &str) -> (String, String) {
    match (User::from_request(request), Client::connect(DB_URL, NoTls)) {
        (Ok(user), Ok(mut client)) => {
            client.execute(
                "INSERT INTO users (name, email) VALUES ($1, $2)",
                &[&user.name, &user.email],
            ).unwrap();
            (OK_RESPONSE.to_string(), "User created".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

pub fn handle_get_request(request: &str) -> (String, String) {
    let user_id = get_id_from_request(request);
    match (user_id.parse::<i32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) => {
            match client.query_one("SELECT id, name, email FROM users WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let user = User {
                        id: row.get(0),
                        name: row.get(1),
                        email: row.get(2),
                    };
                    (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
                }
                Err(_) => (NOT_FOUND.to_string(), "User not found".to_string()),
            }
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

pub fn handle_get_all_request(_request: &str) -> (String, String) {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut users = Vec::new();
            for row in client.query("SELECT id, name, email FROM users", &[]).unwrap() {
                users.push(User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                });
            }
            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

pub fn handle_put_request(request: &str) -> (String, String) {
    let user_id = get_id_from_request(request);
    match (
        user_id.parse::<i32>(),
        User::from_request(request),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(id), Ok(user), Ok(mut client)) => {
            client.execute(
                "UPDATE users SET name = $1, email = $2 WHERE id = $3",
                &[&user.name, &user.email, &id],
            ).unwrap();
            (OK_RESPONSE.to_string(), "User updated".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

pub fn handle_delete_request(request: &str) -> (String, String) {
    let user_id = get_id_from_request(request);
    match (user_id.parse::<i32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client.execute("DELETE FROM users WHERE id = $1", &[&id]).unwrap();
            if rows_affected == 0 {
                return (NOT_FOUND.to_string(), "User not found".to_string());
            }
            (OK_RESPONSE.to_string(), "User deleted".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

// Helper function to extract ID from request
fn get_id_from_request(request: &str) -> &str {
    request.split('/').nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}
