pub mod mega;

pub struct DownloadItem {
    pub(crate) url: String,
    pub name: String,
    pub size: u64,
    pub host: String,
}

pub trait HostPlugin {
    async fn get_download_data(url: String) -> Result<DownloadItem, Box<dyn std::error::Error>>;
}
