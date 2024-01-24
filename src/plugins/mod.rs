pub mod mega;

#[derive(Default)]
pub struct DownloadItem {
    pub url: String,
    pub name: String,
    pub size: u64,
    pub host: String,
}

pub trait HostPlugin {
    const REGEX: &'static str;
    async fn get_download_data(url: String) -> Result<DownloadItem, Box<dyn std::error::Error>>;
    async fn validate_url(url: &str) -> bool {
        let re = regex::Regex::new(Self::REGEX).unwrap();
        re.is_match(&url)
    }
}
