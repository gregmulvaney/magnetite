pub mod mega;

pub struct DownloadItem {
    url: String,
    name: String,
    size: u64,
    host: String,
}

pub trait HostPlugin {
    async fn get_download_data(
        &self,
        url: &str,
    ) -> Result<DownloadItem, Box<dyn std::error::Error>>;
}
