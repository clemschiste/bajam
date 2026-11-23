use axum::{
    Extension, extract::{Request, State}, http::{Method, StatusCode}, middleware::Next, response::Response,
};
use url::Url;

use crate::{middleware::auth::{AuthContext, AuthMethod}, state::AppState};

// CSRF middleware
pub async fn middleware_origin(
  State(state): State<AppState>,
  Extension(auth_context): Extension<AuthContext>,
  req: Request,
  next: Next,
) -> Result<Response, StatusCode> {

  if matches!(
      *req.method(),
      Method::GET | Method::HEAD | Method::OPTIONS
  ) {
      return Ok(next.run(req).await);
  }

  match auth_context.auth_method {
    AuthMethod::Bearer => Ok(next.run(req).await),
    AuthMethod::Cookie => {
      let config = state.config;
      let headers = req.headers();

      let url = headers
        .get("Origin").or_else(|| headers.get("Referer"))
        .and_then(|value| value.to_str().ok());

      let extracted_origin = match url {
        Some(url) => extract_origin(url),
        None => return Err(StatusCode::FORBIDDEN)
      };

      dbg!(&extracted_origin);

      if extracted_origin == Some(config.allowed_origin) {
        Ok(next.run(req).await)
      } else {
        Err(StatusCode::FORBIDDEN)
      }
    }
  }
}

fn extract_origin(value: &str) -> Option<String> {
    let url = Url::parse(value).ok()?;

    Some(format!(
        "{}://{}",
        url.scheme(),
        url[url::Position::BeforeHost..url::Position::AfterPort].to_string()
    ))
}
