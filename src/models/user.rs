use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn from_request(request: &str) -> Result<User, serde_json::Error> {
        serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
    }
}
