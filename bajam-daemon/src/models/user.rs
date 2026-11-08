use serde::{Deserialize, Serialize};
// Sql struct for query as
#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

// Data coming from user at login
#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterPost {
    pub username: String,
    pub password: String,
}
