use askama::Template;
use axum::Form;
use axum::response::Html;
use axum::{http::StatusCode};
use axum::{Extension, Json, extract::State};
use bajam_common::models::escapade::Escapade;
use sqlx::Result;
use serde::{Deserialize, Serialize};

use crate::db::escapade_queries::sql_list_escapades;
use crate::middleware::auth::AuthContext;
use crate::state::AppState;

// Maybe add description later ?
#[derive(Serialize, Deserialize, Debug)]
pub struct EscapadePost {
  name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EscapadeResponse {
  pub escapade_id: i64,
}
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
  Json(payload): Json<EscapadePost>,
) -> Result<Json<EscapadeResponse>, (StatusCode, String)> {

    let escapade_response = escapade_post_tx(&state, auth_context.user_id, payload.name).await?;

    Ok(Json(escapade_response))
}

pub async fn escapade_post_web(
  State(state): State<AppState>,
  Extension(auth_context): Extension<AuthContext>,
  Form(payload): Form<EscapadePost>,
) -> Result<Html<String>, (StatusCode, String)> {

    escapade_post_tx(&state, auth_context.user_id, payload.name).await?;
    let escapade_list = sql_list_escapades(&state, &auth_context.user_id).await?;

    let template = EscapadesListTemplate { escapades: escapade_list };
    let html = template.render().map_err(|e| {
      (
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("Template error: {e}"),
      )
    })?;
    
    Ok(Html(html))
}


async fn escapade_post_tx(
    state: &AppState,
    user_id: i64,
    escapade_name: String
) -> Result<EscapadeResponse, (StatusCode, String)> {
    // Début de transaction (avec begin)
    let mut tx = state.db.begin().await.map_err(|e| {
        eprintln!("Database error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    // 1. Création de l'escapade et récupération de son ID -> query scalar pour récup l'unique valeur retournée
    let escapade_id: i64 = sqlx::query_scalar(
        r#"
            INSERT INTO escapades (name)
            VALUES (?)
            RETURNING id
        "#
    )
    .bind(escapade_name)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    // 2. Association entre l'utilisateur et l'escapade
    sqlx::query(
        r#"
            INSERT INTO escapades_map (escapade_id, user_id, role)
            VALUES (?, ?, 'admin')
        "#
    )
    .bind(escapade_id)
    .bind(user_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    // 3. Validation de la transaction
    tx.commit().await.map_err(|e| {
        eprintln!("Database error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    Ok(EscapadeResponse { escapade_id })
    
}
