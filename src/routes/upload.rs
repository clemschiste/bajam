use axum::{
    body::Bytes,
};
use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::Response,
};

use image::ImageReader;
use std::io::Cursor;

use crate::AppState;


// Il faut check le format de la picture d'abord ?
// Ce sera post_picture ensuite
pub async fn post_picture(
    State(state): State<AppState>,
    Path(picture_id): Path<String>,
    body: Bytes
) -> Result<StatusCode, (StatusCode, String)> {

// 1. Limite de taille
    if body.len() > 10 * 1024 * 1024 {
        return Err((
            StatusCode::PAYLOAD_TOO_LARGE,
            "Image too large".into(),
        ));
    }

    // 2. Vérification que c'est réellement une image
    let image = ImageReader::new(Cursor::new(&body))
        .with_guessed_format()
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Invalid image: {e}"),
            )
        })?
        .decode()
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Invalid image: {e}"),
            )
        })?;

    // 3. Vérifier éventuellement les dimensions
    if image.width() > 4096 || image.height() > 4096 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Image dimensions are too large".into(),
        ));
    }
    
    let path = format!("./uploads/{}.jpg", picture_id);
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
    sqlx::query("INSERT INTO pictures (picture_id) VALUES (?)")
        .bind(picture_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            eprintln!("Database error {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error : {e}"))
        })?;

    println!("Received {} bytes", body.len());

    Ok(StatusCode::NO_CONTENT)

}

pub async fn get_picture(
    Path(upload_id): Path<String>,
) -> Result<Response, StatusCode> {
    let path = format!("./uploads/{}.jpg", upload_id);

    println!("Recherche de la picture : {}", path);

    let data = tokio::fs::read(&path)
        .await
        .map_err(|e| {
            println!("Erreur lecture picture : {}", e);
            StatusCode::NOT_FOUND
        })?;

    println!("picture trouvée : {} octets", data.len());

    Response::builder()
        .header(header::CONTENT_TYPE, "image/jpeg")
        .body(data.into())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

