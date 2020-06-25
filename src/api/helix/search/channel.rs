
use std::error::Error;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use reqwest::Method;

use crate::api::{ApiEndpoint, helix::{result::ApiResult, pagination::ForwardPagination}};
use super::SearchGroup;

#[derive(Debug, Default, Serialize)]
pub struct ChannelSearchRequest {
    query: String,
    first: Option<u8>,

    #[serde(flatten)]
    pagination: ForwardPagination
}

impl ChannelSearchRequest {
    pub fn with_query<S: Into<String>>(mut self, query: S) -> Self {
        self.query = query.into();
        self
    }

    pub fn with_first(mut self, first: Option<u8>) -> Self {
        self.first = first;
        self
    }

    pub fn with_after<S: Into<String>>(mut self, after: Option<S>) -> Self {
        self.pagination.after = after.map(|x| x.into());
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct ChannelResponse {
    pub game_id: String,
    pub id: String,
    pub display_name: String,
    pub broadcaster_language: String,
    pub title: String,
    pub thumbnail_url: String,
    pub is_live: bool,
    pub started_at: String,
    pub tag_ids: Vec<String>
}

#[async_trait(?Send)]
pub trait ChannelGroup {
    async fn find_channel(&self, request: ChannelSearchRequest) -> Result<ApiResult<ChannelResponse>, Box<dyn Error>>;
}

#[async_trait(?Send)]
impl ChannelGroup for SearchGroup {
    async fn find_channel(&self, request: ChannelSearchRequest) -> Result<ApiResult<ChannelResponse>, Box<dyn Error>> {
        Ok(self.client.call(ApiEndpoint::Helix, Method::GET, None, "search/channels", &request).await?)
    }
}


