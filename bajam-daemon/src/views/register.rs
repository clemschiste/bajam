use askama::Template;
use axum::{http::StatusCode, response::Html};

#[derive(Template)] // this will generate the code...
#[template(path = "register.html")]
struct RegisterTemplate {}

pub async fn register_view() -> Result<Html<String>, (StatusCode, String)> {
    let template = RegisterTemplate {};
    let html = template.render().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR,
        format!("Template error: {e}"))
    })?;

    Ok(Html(html))
}
