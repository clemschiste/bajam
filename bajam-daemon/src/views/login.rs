use askama::Template;
use axum::{http::StatusCode, response::Html};

#[derive(Template)] // this will generate the code...
#[template(path = "login.html")]
struct LoginTemplate {}

pub async fn login_view() -> Result<Html<String>, (StatusCode, String)> {
    let template = LoginTemplate {};
    let html = template.render().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR,
        format!("Template error: {e}"))
    })?;

    Ok(Html(html))
}
