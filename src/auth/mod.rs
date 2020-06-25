
use std::{time::Duration, error::Error};
use tokio::time::Instant;
use async_trait::async_trait;

mod static_token;
mod client_credentials;

pub use static_token::*;
pub use client_credentials::*;

#[derive(Default)]
pub struct AccessToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<Duration>,
    pub scopes: Option<Vec<String>>,
    pub time: Option<Instant>
}

#[async_trait(?Send)]
pub(crate) trait AuthProvider {
    fn get_client_id(&self) -> Option<String>;
    fn can_refresh(&self) -> bool;
    async fn get_token(&self, scopes: Option<Vec<String>>) -> Result<AccessToken, Box<dyn Error>>;
    async fn refresh_token(&self, access_token: &mut AccessToken, scopes: Option<Vec<String>>) -> Result<(), Box<dyn Error>>;
}
