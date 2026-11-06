use axum::{http::StatusCode};
use axum::{Extension, Json, extract::State};
use sqlx::Result;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

pub async fn escapade_post(
  State(state): State<AppState>,
  Extension(user_id): Extension<i64>,
  Json(payload): Json<EscapadePost>,
) -> Result<Json<EscapadeResponse>, (StatusCode, String)> {
  
    println!("Escapade creation request received");

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
    .bind(payload.name)
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
            VALUES (?, ?, 'creator')
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

    Ok(Json(EscapadeResponse{escapade_id}))
}

// Maybe add description later ?
#[derive(Serialize, Deserialize, Debug)]
pub struct EscapadePost {
  name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EscapadeResponse {
  pub escapade_id: i64,
}
