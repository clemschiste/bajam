pub mod heartbeat_post;
pub mod index;
pub mod upload;
pub mod user;

use serde::{Deserialize, Serialize};
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use std::time::Instant;

// SQL struct
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heartbeat {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: String,
    pub description: Option<String>,
    pub picture_id: Option<String>,
}
// Heartbeat response. Inclut un historique des positions
#[derive(Serialize, Deserialize, Debug)]
pub struct HeartbeatResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: String,
    pub description: Option<String>,
    pub picture_id: Option<String>,
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

// La response fait transiter la requete vers la route méthode.
// Tout ce qu'il y a avant est prévu pour check et intercepter avec une
// Err(StatusCode::UNAUTHORIZED).
// Il faut donc lire la requete et notamment son header Authorization: Bearer *UUid*
// Décode le token, check la db -> get user_id
pub async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {

    // Logique
    
    Ok(next.run(req).await)
}
