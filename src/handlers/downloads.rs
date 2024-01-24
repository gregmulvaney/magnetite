use crate::{
    entities,
    entities::prelude::Download,
    plugins::{mega::MegaPlugin, DownloadItem, HostPlugin},
    AppState,
};
use axum::{debug_handler, extract::State, Json};
use regex::Regex;
use sea_orm::{ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct AddDownloadsRequest {
    url: String,
}

#[debug_handler]
pub async fn add_downloads_handler(
    state: State<AppState>,
    Json(payload): Json<AddDownloadsRequest>,
) {
    // Parse url into a URL object
    let parsed_url = Url::parse(&payload.url).unwrap();
    let mut download: DownloadItem = ();
    match parsed_url.host_str().unwrap() {
        "mega.nz" => {
            let re = Regex::new(MegaPlugin::default().regex).unwrap();
            if re.is_match(&payload.url) {
                download = MegaPlugin::get_download_data(payload.url).await.unwrap();
            } else {
                tracing::error!("Invalid mega.nz url");
            }
        }
        _ => {
            unimplemented!("Unknown plugin")
        }
    }

    let model = entities::download::ActiveModel {
        url: Set(download.url),
        name: Set(download.name),
        size: Set(download.size as i32),
        host: Set(download.host),
        status: Set("pending".to_string()),
        ..Default::default()
    };

    Download::insert(model).exec(&state.db).await.unwrap();
}

#[debug_handler]
pub async fn get_downloads_handler() -> Json<String> {
    Json("Get downloads handler".to_string())
}
