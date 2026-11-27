use askama::Template;
use axum::Form;
use axum::response::Html;
use axum::{http::StatusCode};
use axum::{Extension, Json, extract::State};
use bajam_common::models::escapade::Escapade;
use sqlx::Result;

use crate::escapades::error::to_http;
use crate::escapades::create::service::create_escapade;
use crate::escapades::escapade::EscapadePost;
use crate::escapades::list::service::list_escapades;

use crate::auth::middleware::AuthContext;
use crate::state::AppState;

#[derive(Template)]
#[template(path = "escapade_list.html")]
pub struct EscapadesListTemplate {
    pub escapades: Vec<Escapade>,
}

// For posting an escapade we create a transaction for the two tables escapades and escapade_map
// We must insert the user_id + escapade_id in the second
pub async fn escapade_post_api(
  State(state): State<AppState>,
  Extension(auth_context): Extension<AuthContext>,
  Json(escapade): Json<EscapadePost>,
) -> Result<(), (StatusCode, String)> {

    create_escapade(&state.db, auth_context.user_id, escapade.name().to_string()).await.map_err(to_http)?;

    Ok(())
}

pub async fn escapade_post_web(
  State(state): State<AppState>,
  Extension(auth_context): Extension<AuthContext>,
  Form(payload): Form<EscapadePost>,
) -> Result<Html<String>, (StatusCode, String)> {

    create_escapade(&state.db, auth_context.user_id, payload.name).await.map_err(to_http)?;
    let escapade_list = list_escapades(&state.db, auth_context.user_id).await.map_err(to_http)?;

    let template = EscapadesListTemplate { escapades: escapade_list };
    let html = template.render().map_err(|e| {
      (
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("Template error: {e}"),
      )
    })?;
    
    Ok(Html(html))
}
