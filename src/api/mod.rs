
use std::error::Error;
use reqwest::Url;

#[cfg(feature = "helix")]
pub mod helix;

pub(crate) enum ApiEndpoint {
    #[cfg(feature = "helix")]
    Helix,
    Auth,
    #[allow(dead_code)]
    Custom(String)
}

impl ApiEndpoint {
    pub(crate) fn get_endpoint(&self, url: &str) -> Result<Url, Box<dyn Error>> {
        let endpoint: Url = Url::parse(match self {
            #[cfg(feature = "helix")]
            ApiEndpoint::Helix => "https://api.twitch.tv/helix/",
            ApiEndpoint::Auth => "https://id.twitch.tv/oauth2/",
            ApiEndpoint::Custom(endpoint) => endpoint,
        })?;
        Ok(endpoint.join(url)?)
    }
}
