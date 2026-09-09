pub mod heartbeat_post;
pub mod index;
pub mod upload;

use serde::{Deserialize, Serialize};
use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;

// SQL struct
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heartbeat {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: String,
    pub description: Option<String>,
    pub upload_id: Option<String>,
}
// Heartbeat response. Inclut un historique des positions
#[derive(Serialize, Deserialize, Debug)]
pub struct HeartbeatResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: String,
    pub description: Option<String>,
    pub upload_id: Option<String>,
    pub history: Vec<Heartbeat>, // Pour map libre js -> [longitude, latitude]
}

pub async fn logger(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri    = req.uri().clone();
    let start  = Instant::now();
    // call next and get the response
    let response = next.run(req).await;
    // now we have the status code and elapsed time
    println!(
        "{} {} {} {}ms",
        method, uri,
        response.status(),
        start.elapsed().as_millis()
    );
    // prints: GET /users 200 OK 4ms
    response
}

