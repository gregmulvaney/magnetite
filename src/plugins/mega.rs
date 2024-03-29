use crate::plugins::{DownloadItem, HostPlugin};
use std::error::Error;

pub struct MegaPlugin {}

impl HostPlugin for MegaPlugin {
    const REGEX: &'static str = r#"(https?:\/\/)?(www\.)?mega\.nz\/(file|folder)\/[a-zA-Z0-9#!_-]{8,11}#?[a-zA-Z0-9#!_-]{0,8}"#;

    // TODO: Handle dead links and other errors
    async fn get_download_data(url: String) -> Result<DownloadItem, Box<dyn Error>> {
        let http_client = reqwest::Client::new();
        let mega_client = mega::Client::builder().build(http_client).unwrap();
        let nodes = mega_client.fetch_public_nodes(&url).await?;
        let name = nodes.roots().next().unwrap().name().to_string();
        let mut size: u64 = 0;
        for node in nodes {
            size += node.size();
        }
        Ok(DownloadItem {
            url,
            name,
            size,
            host: "mega".to_string(),
        })
    }
}
