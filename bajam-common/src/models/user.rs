use serde::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}
