use axum::{
    body::Bytes,
    Json
};
use serde::Serialize;
use uuid::Uuid;
use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::Response,
};

use crate::AppState;

#[derive(Serialize)]
pub struct UploadResponse {
    pub upload_id: Uuid,
}

// Il faut check le format de la photo d'abord ?
// Ce sera post_photo ensuite
pub async fn post_file(
    State(state): State<AppState>,
    body: Bytes,
) -> Result<Json<UploadResponse>, (StatusCode, String)> {
    let upload_id = Uuid::new_v4();
    let path = format!("./uploads/{}.jpg", upload_id);
    tokio::fs::create_dir_all("./uploads")
        .await
        .map_err(|e| {
            eprintln!("{e}");
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())})?;

    tokio::fs::write(&path, &body)
        .await
        .map_err(|e| {
            eprintln!("{e}");
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())})?;
        

    // Enregistre l'upload pour que les heartbeats puissent le référencer (FK)
    sqlx::query("INSERT INTO photos (upload_id) VALUES (?)")
        .bind(upload_id.to_string())
        .execute(&state.db)
        .await
        .map_err(|e| {
            eprintln!("Database error {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error : {e}"))
        })?;

    println!("Received {} bytes", body.len());

    Ok(Json(UploadResponse {
        upload_id,
    }))
}

pub async fn get_photo(
    Path(upload_id): Path<String>,
) -> Result<Response, StatusCode> {
    let path = format!("./uploads/{}.jpg", upload_id);

    println!("Recherche de la photo : {}", path);

    let data = tokio::fs::read(&path)
        .await
        .map_err(|e| {
            println!("Erreur lecture photo : {}", e);
            StatusCode::NOT_FOUND
        })?;

    println!("Photo trouvée : {} octets", data.len());

    Response::builder()
        .header(header::CONTENT_TYPE, "image/jpeg")
        .body(data.into())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

