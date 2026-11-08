use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;

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
