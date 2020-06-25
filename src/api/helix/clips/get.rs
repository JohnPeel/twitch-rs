
use std::error::Error;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use reqwest::Method;

use crate::{api::{ApiEndpoint, helix::{result::ApiResult, pagination::*}}, util::extend_url};
use super::ClipsGroup;

#[derive(Debug, Default, Serialize)]
pub struct GetClipsRequest {
    broadcaster_id: Option<String>,
    game_id: Option<String>,

    #[serde(skip_serializing)]
    ids: Option<Vec<String>>,

    first: Option<u8>,
    started_at: Option<String>,
    ended_at: Option<String>,

    #[serde(flatten)]
    pagination: Pagination
}

impl GetClipsRequest {
    pub fn with_broadcaster_id<S: Into<String>>(mut self, broadcaster_id: S) -> Self {
        self.broadcaster_id = Some(broadcaster_id.into());
        self.game_id = None;
        self.ids = None;
        self
    }

    pub fn with_game_id<S: Into<String>>(mut self, game_id: S) -> Self {
        self.broadcaster_id = None;
        self.game_id = Some(game_id.into());
        self.ids = None;
        self
    }

    pub fn with_ids<S: Into<String>>(mut self, id: Vec<S>) -> Self {
        self.broadcaster_id = None;
        self.game_id = None;
        self.ids = Some(id.into_iter().map(|x| x.into()).collect());
        self
    }

    pub fn with_first(mut self, first: Option<u8>) -> Self {
        self.first = first;
        self
    }

    pub fn with_started_at<S: Into<String>>(mut self, started_at: Option<S>) -> Self {
        self.started_at = started_at.map(|x| x.into());
        self
    }

    pub fn with_ended_at<S: Into<String>>(mut self, ended_at: Option<S>) -> Self {
        self.ended_at = ended_at.map(|x| x.into());
        self
    }

    pub fn with_after<S: Into<String>>(mut self, after: Option<S>) -> Self {
        self.pagination.forward = Some(ForwardPagination {
            after: after.map(|x| x.into())
        });
        self.pagination.backward = None;
        self
    }

    pub fn with_before<S: Into<String>>(mut self, before: Option<S>) -> Self {
        self.pagination.forward = None;
        self.pagination.backward = Some(BackwardPagination {
            before: before.map(|x| x.into())
        });
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetClipsResponse {
    pub broadcaster_id: String,
    pub broadcaster_name: String,
    pub created_at: String,
    pub creator_id: String,
    pub creator_name: String,
    pub embed_url: String,
    pub game_id: String,
    pub id: String,
    pub language: String,
    pub thumbnail_url: String,
    pub title: String,
    pub url: String,
    pub video_id: String,
    pub view_count: u64
}

#[async_trait(?Send)]
pub trait GetClips {
    async fn get_clips(&self, request: GetClipsRequest) -> Result<ApiResult<GetClipsResponse>, Box<dyn Error>>;
}

#[async_trait(?Send)]
impl GetClips for ClipsGroup {
    async fn get_clips(&self, request: GetClipsRequest) -> Result<ApiResult<GetClipsResponse>, Box<dyn Error>> {
        Ok(self.client.call(ApiEndpoint::Helix, Method::GET, None, extend_url("clips", "id", &request.ids)?, &request).await?)
    }
}
