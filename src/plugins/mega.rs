use crate::plugins::{DownloadItem, HostPlugin};
use std::error::Error;

pub struct MegaPlugin {
    url: String,
}

impl HostPlugin for MegaPlugin {
    async fn get_download_data(&self, url: &str) -> Result<DownloadItem, Box<dyn Error>> {
        unimplemented!("Mega plugin")
    }
}
