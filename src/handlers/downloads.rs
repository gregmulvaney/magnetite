use crate::{
    entities,
    entities::prelude::Download,
    plugins::{mega::MegaPlugin, DownloadItem, HostPlugin},
    AppState,
};
use axum::{debug_handler, extract::State, Json};
use chrono::Local;
use sea_orm::{ActiveModelBehavior, ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct AddDownloadsRequest {
    urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddDownloadsResponse {
    url: String,
    status: i32,
}

#[debug_handler]
pub async fn add_downloads_handler(
    state: State<AppState>,
    Json(payload): Json<AddDownloadsRequest>,
) -> Json<Vec<AddDownloadsResponse>> {
    let mut results = Vec::new();
    for url in payload.urls {
        let parsed_url = Url::parse(&url).unwrap();
        let host = parsed_url.host_str().unwrap();
        // Initialize empty DownloadItem variable
        let mut download: DownloadItem = Default::default();
        // TODO: Extract plugin matching
        match host {
            "mega.nz" => {
                if MegaPlugin::validate_url(&url).await {
                    download = MegaPlugin::get_download_data(url.to_string())
                        .await
                        .unwrap();
                }
            }
            _ => {
                results.push(AddDownloadsResponse {
                    url: url.to_string(),
                    status: 400,
                });
                break;
            }
        }
        let model = entities::download::ActiveModel {
            name: Set(download.name),
            url: Set(url.to_string()),
            host: Set(download.host),
            size: Set(download.size as i32),
            status: Set("pending".to_owned()),
            created_at: Set(Local::now().naive_local()),
            ..Default::default()
        };
        Download::insert(model).exec(&state.db).await.unwrap();
        let result = AddDownloadsResponse { url, status: 200 };
        results.push(result)
    }
    Json(results)
}

#[debug_handler]
pub async fn get_downloads_handler() -> Json<String> {
    Json("Get downloads handler".to_string())
}
