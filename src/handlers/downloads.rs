use axum::{debug_handler, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddDownloadsRequest {
    url: String,
}

#[debug_handler]
pub async fn add_downloads_handler(Json(payload): Json<AddDownloadsRequest>) {
    unimplemented!("Add downloads handler")
}

#[debug_handler]
pub async fn get_downloads_handler() -> Json<String> {
    Json("Get downloads handler".to_string())
}
