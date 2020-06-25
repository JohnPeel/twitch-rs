
use std::error::Error;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use reqwest::Method;

use crate::api::{ApiEndpoint, helix::{result::ApiResult, pagination::ForwardPagination}};
use super::SearchGroup;

#[derive(Debug, Default, Serialize)]
pub struct CategorySearchRequest {
    query: String,
    first: Option<u8>,

    #[serde(flatten)]
    pagination: ForwardPagination
}

impl CategorySearchRequest {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryResponse {
    pub box_art_url: String,
    pub name: String,
    pub id: String
}

#[async_trait(?Send)]
pub trait CategoryGroup {
    async fn find_category(&self, request: CategorySearchRequest) -> Result<ApiResult<CategoryResponse>, Box<dyn Error>>;
}

#[async_trait(?Send)]
impl CategoryGroup for SearchGroup {
    async fn find_category(&self, request: CategorySearchRequest) -> Result<ApiResult<CategoryResponse>, Box<dyn Error>> {
        Ok(self.client.call(ApiEndpoint::Helix, Method::GET, None, "search/categories", &request).await?)
    }
}


